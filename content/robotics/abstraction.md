---
page-title: Abstraction
order: 2
---

# Command Based Programming

"Command Based Programming" is the design pattern we use to structure our robot code. We use abstraction to separate our robot into modular chunks.

I like to preface this section by reminding you that robots are complex systems, built from very dumb parts. The "robot" is not at all intelligent, it's just a collection of sensors and actuators that are themselves mostly made of metal and plastic. The most fundamental control of our robot is setting voltages and reading voltages. Everything cool our robots do are built on top of this simple foundation.

> Read more about abstraction in Computer Science: <https://en.wikipedia.org/wiki/Abstraction_(computer_science)>.

In wide strokes, Command Based Programming asks of us to separate our robot into "Subsystems" and "Commands". Subsystems are the 'parts' of the robot, like the drivetrain, the shooter, or the climber. Commands are the 'actions' of the robot, anything the robot does is a accomplished by running a Command.

## Subsystems

The first step to programming a robot is identifying how we should group up all of the sensors and actuators into "Subsystems". In the 2023-2024 season our robot had 14 motors, 4 absolute encoders, 1 gyro/imu, 1 proximity IR sensor, and an LED controller. This short list is all the information our robot's controller has access to, all alone it doesn't convey enough information about how to create the robot's intended behavior. Our first step is assigning each sensor and actuator a "meaning" by grouping them into subsystems.

We designed 7 subsystems for our robot:

- Drivetrain
  - 4 drive motors
  - 4 turn motors
  - 4 absolute encoders
  - 1 gyro/imu
- GroundIntake
  - 1 intake motor
  - 1 proximity IR sensor
- Indexer
  - 1 indexer motor
- Shooter
  - 2 shooter motors
- AmpGuide
  - 1 AmpGuide motor
- Climber
  - 1 climber motor
- LEDs
  - 1 LED controller

Each actuator and sensor is assigned to at-most one subsystem. This disjointedness is a _requirement_ of the Command Based Programming framework. For actuators this is a safety feature, it prevents the robot from doing dangerous things like trying to drive forward and backwards at the same time. For sensors this is just a product of how the underlying control system works and a limitation of the RoboRIO, nonetheless it's a good practice to follow.

Another benefit of this separation is we've created modularity. Modularity is critical in software engineering, and especially in FRC. Since each subsystem is individually responsible for controlling its own actuators and sensors we're able to (ideally) test each subsystem in isolation. This is a great feature to have during the build season when the robot is constantly changing. Being able to test our climber and shooter independently means we'll be able to test them each as soon as they are ready, rather than waiting for the whole robot to be assembled.

### Private vs Public Methods

The biggest constraint in FRC is time. Especially on programming team, as a rule **"building the robot" has a higher priority than "programming the robot"**. At the end of the day, assuming there is enough caffeine we _could_ program the entire robot the night before competition.

Paired with not having enough access to the robot, build team has a habit of making near constant design changes to the robot. One day we'll have a climber with 1 motor and a limit switch and the next it will be 2 motors and a laser rangefinder. One day we have a shooter with 1 flywheel and then the next it's a shooter with 2 flywheels and an indexer. Programming team is working at it's best when we're able to quickly respond to significant design changes.

Fundamentally, changing from a "1-motor climber" to a "2-motor climber" doesn't really change the robot's intended behavior. No matter how many motors are on the climber we still assume the climber is going to _move up_ and _move down_. **A climber could be built with a winch, a scissor lift, a telescoping arm, or a pneumatic piston and it would still _move up_ and _move down_**, it would *_climb_. A shooter that has 1 wheel or 2 wheels still _shoots_. A ground intake that uses CTRE Falcon motors or REV Neo motors still _intakes_.

There's a clear pattern here, no matter how we build a subsystem it typically has the same "intended behavior".

Writing a subsystem is a 2 step process:

1. Write the "Public API" of the subsystem. This is writing stubs for the `public` methods that the subsystem should have.

2. Write the low level implementation. The `private` variables and methods that make the specific hardware work.

> API here means "Application Programming Interface". I may also refer to this idea as the subsystem's _contract_. A contract outlines the responsibilities of the signing parties and how they will work together.

Let's look at an example:

```java
/**
 * ClimberSubsystem controls our climber. It only goes up or down.
 */
public class ClimberSubsystem extends Subsystem {
    // This climber has 1 motor and it's driven with a SparkMAX
    //
    // Note: `private` in Java means the `motor` variable is only accessible within this class.
    // This makes `motor` an "implementation detail"
    private CANSparkMax motor = new CANSparkMax(ClimberConstants.kClimberMotor);

    public Climber() {
        motor.setIdleMode(IdleMode.kBrake);
    }

    private extend() {
        // Runs at +50%
        motor.set(0.5);
    }

    private retract() {
        // Runs at -50%
        motor.set(-0.5);
    }

    private stop() {
        // Run at 0%
        motor.set(0);
    }

    /**
     * Returns a command that extends the climber while it is ran
     *
     * @return the {@link Command}
     */
    public Command cExtend() {
        // We'll go deeper how to build commands later. Here you just need to know
        // that `extend` is called while the button is held, then when the button
        // is released `stop` is called.
        return runEnd(this::extend, this::stop);
    }

    /**
     * Returns a command that retracts the climber while it is ran
     *
     * @return the {@link Command}
     */
    public Command cRetract() {
        return runEnd(this::retract, this::stop);
    }
}

/**
 * RobotContainer is where we connect driver controllers to Subsystems and Commands.
 */
public class RobotContainer {
    private final ClimberSubsystem climber = new ClimberSubsystem();
    private final CommandPS5Controller m_driverController =
        new CommandPS5Controller(OperatorConstants.kDriverControllerPort);

    public RobotContainer() {
        // d-pad up extends the climber
        m_driverController.povUp().whileTrue(climber.cExtend());
        // d-pad down retracts the climber
        m_driverController.povDown().whileTrue(climber.cRetract());
    }
}
```

> This short program defines a *real* FRC Robot.

Let's update this design. Say we test the code on our robot and we learn that the climber is far to weak to climb. The first change we make is increasing the power of the climber motor from 50% to 100% but sadly, this doesn't fix the problem, we send it back to build team...

Build team decides that the best way to fix this issue is to add a second motor to the climber's axle. This is a decent choice, adding a motor on the opposite side of an axle is an quick and easy way to increase a mechanism's torque.

Here's how we could change the code to add another motor and increase the power to 100%.

```diff
/**
 * ClimberSubsystem controls our climber. It only goes up or down.
 */
public class ClimberSubsystem extends Subsystem {
    private CANSparkMax motor = new CANSparkMax(ClimberConstants.kClimberMotor);
+   private CANSparkMax motor2 = new CANSparkMax(ClimberConstants.kClimberMotor2);

    public Climber() {
        motor.setIdleMode(IdleMode.kBrake);
+       motor2.setIdleMode(IdleMode.kBrake);
    }

    private extend() {
        motor.set(1.0);
+       motor2.set(-1.0);
    }

    private retract() {
        motor.set(-1.0);
+       motor2.set(1.0);
    }

    private stop() {
        motor.set(0);
+       motor2.set(0);
    }

    /**
     * Returns a command that extends the climber while it is ran
     *
     * @return the {@link Command}
     */
    public Command cExtend() {
        return runEnd(this::extend, this::stop);
    }

    /**
     * Returns a command that retracts the climber while it is ran
     *
     * @return the {@link Command}
     */
    public Command cRetract() {
        return runEnd(this::retract, this::stop);
    }
}

/**
 * RobotContainer is where we connect driver controllers to Subsystems and Commands.
 */
public class RobotContainer {
    private final ClimberSubsystem climber = new ClimberSubsystem();
    private final CommandPS5Controller m_driverController =
        new CommandPS5Controller(OperatorConstants.kDriverControllerPort);

    public RobotContainer() {
        m_driverController.povUp().whileTrue(climber.cExtend());
        m_driverController.povDown().whileTrue(climber.cRetract());
    }
}
```

Notice a few things:

1. `RobotContainer` has _no changes_. We have not changed the meaning of the climber, it still "extends" and "retracts".
2. The signature of `ClimberSubsystem`'s public methods hasn't changed either. Anything/anyone relying or using the climber would not need to update how they are using the climber.
3. Within the `ClimberSubsystem` we had a sufficient level of abstraction that implementation was easy to write and straightforward to understand. How hard would it be to add another pair of motors?

### Subsystems Summary

- Subsystems are the _nouns_ of our robot.
- Subsystems should be modular and subsystems should be disjoint.
- Subsystems should promote high level abstractions, such that when an implementation detail changes the rest of the code should not be affected.

## Commands

If **subsystems** represent the _nouns_ of our robot then **commands** represent the _verbs_.

Say you're describing your robot to a parent or a sponsor. You might say things like "our robot has a climber" and "the climber extends and retracts". The first statement implies the existence of `ClimberSubsystem` - something that defines the climber. The second statement implies the existence of 2 commands, "climber extend" and "climber retract".

Commands in WPILib have been design to work well at any depth. Complex statements like "during autonomous the robot drives forward and then picks up the nearest 2 game pieces and scores them" also implies the existence of another Command "drive forward and score 2". **Any action the robot preforms must be represented by a single Command**.

### Scheduling

Commands are ran by the "Command Scheduler". By default the scheduler runs every 20ms (50 Hz) and each "tick" running commands are processed. The scheduler enforces a very simple rule **"each `Subsystem` can only be required by at most 1 running `Command`"**. Once again this is a safety feature. It prevents the robot from doing dangerous things like trying to drive forward and backwards at the same time.

> This "drive forward and backwards at the same time" story is a parable for an entire class of bugs in Computer Science called [Race conditions](https://en.wikipedia.org/wiki/Race_condition). Robot's `Command`s and `Subsystem`s run concurrently and to prevent race conditions the Command Scheduler enforces "mutual exclusion" on `Subsystem`s.

The Command Scheduler is _not_ magic, `Command`s must communicate their `Subsystem` requirements to the scheduler, failing to do so will result in badness. If two `Command`s require the same `Subsystem` then the scheduler will "cancel" one of the commands before running the other.

When dealing with any kind of "scheduler" it's nice to use Gantt Charts to help visualize timelines. On these charts the horizontal axis represents time and each subsystem gets it's own row. `Command` are represented as blocks of time where the subsystem is being required.

Here's a simple example:

![Single Requirement Gantt Chart](/static/img/gantt.png)

**Notice how none of the Red blocks overlap**. Overlapping commands are forbidden. This next chart shows an impossible schedule:

![Invalid Schedule](/static/img/gantt-1.png)

- Commands D and E are in conflict
- Commands C and F are in conflict

When presented with this situation the scheduler has to make a choice about which command to run. This choice is defined by the first command's [interrupt behavior](https://github.wpilib.org/allwpilib/docs/release/java/edu/wpi/first/wpilibj2/command/Command.InterruptionBehavior.html)

By default the scheduler will `interrupt` the existing command and start running the new command, this is called `kCancelSelf`.

Important commands can be marked as `kCancelIncoming` which will ignore the new command and continue running the existing command. You should use `kCancelIncoming` sparingly, especially during teleop. Cancelling new commands leads to situations where the robot feels unresponsive to the driver. Typically, we'd rather communicate to the driver any kind of pre-condition required to safely operate the robot, rather than forbidding them from doing something. Let them make risky decisions when it makes sense.

> Winning an important match is almost always worth the risk of damaging a replaceable part of the robot.

The following charts demonstrate the scheduler's behavior when faced with a conflict:

![kCancelSelf](/static/img/gantt-2.png)

![kCancelIncoming](/static/img/gantt-3.png)

> Notice how we lost E and C.

#### Miscellaneous Notes

Commands requiring multiple subsystems are allowed, and encouraged! Commands F and G in the above charts are examples.

A command can safely require 0 subsystems. `Commands.waitSeconds(double t)` is a common example of a command that doesn't require any subsystems.

### Lifecycle of a `Command`

`Command`s are [state machines](https://en.wikipedia.org/wiki/Finite-state_machine) representing a complete action to be performed by the robot. The state machine is built from 4 states: `initialize`, `execute`, `isFinished`, and `end`.

1. `void initialize()` - Called once when the command is started
2. `void execute()` - Called every tick while the command is running
3. `boolean isFinished()` - Called after `execute()`, returns `true` when the command should end naturally
4. `void end(boolean interrupted)` - Called once when the command is ended. `interrupted` is `true` if the command was canceled by the scheduler, `false` if the command ended normally because `isFinished()` returned `true`.

![alt text](/static/img/gantt-4.png)

> When referring to things "that come into and out of existence" programmers will use euphemisms like "life-cycle", "born", "died", or "killed". This is to just convey this relatively complicated concept in a way that we're unforuantely already familiar with.

_Back in my day_ all commands were written explicitly by defining these 4 methods. I had a rule on the team (that is still enforced) that every command defined with this pattern must write all 4 methods instead of relying on the default implementations. For the most part it is not longer necessary to write commands this way, nowadays WPILib has a lot of helper classes to make writing commands easier. Nontheless I believe they are still a good starting poing for getting comfortable with Commands.

#### Long-form Command Examples

While demonstrating the 'old way' of writing commands I also want to introduce you to 3 common patterns we use when thinking about commands.

- Infinite Commands
- Finite Commands
- Instantaneous Commands

As you'll see one of the most important parts of a command is the behavior of `isFinished()`.

```java
/**
 * While this command is running extend the climber
 */
public class ExtendClimberCommand extends Command {
    private final ClimberSubsystem climber;

    public ExtendClimberCommand(ClimberSubsystem climber) {
        this.climber = climber;
        addRequirements(climber);
    }

    @Override
    public void initialize() {
        climber.extend();
    }

    @Override
    public void execute() {
    }

    @Override
    public boolean isFinished() {
        return false; // Notice!
    }

    @Override
    public void end(boolean interrupted) {
        climber.stop();
    }
}
```

`ExtendClimberCommand` here is an "Infinite Command". It runs forever, or until something else interrupts it. Often, the interruption is triggered by "releasing a button" or "hitting a limit switch".

> [`Trigger`s](https://github.wpilib.org/allwpilib/docs/release/java/edu/wpi/first/wpilibj2/command/button/Trigger.html) are actually another abstraction that's apart of the WPILib command-based framework. Driver controllers or sensors can be mapped into `Trigger`s which can be used to start and stop commands.

```java
/**
 * Uess a simple control loop to moves the climber to a specific height.
 */
public class ClimberToHeightCommand extends Command {
    private static final double kTolerance = 0.1;

    private final ClimberSubsystem climber;
    private final double height;

    public ClimberToHeightCommand(ClimberSubsystem climber, double height) {
        this.climber = climber;
        this.height = height;
        addRequirements(climber);
    }

    @Override
    public void initialize() {
    }

    @Override
    public void execute() {
        if (climber.getHeight() < height) {
            climber.extend();
        } else {
            climber.retract();
        }
    }

    @Override
    public boolean isFinished() {
        return Math.abs(climber.getHeight() - height) < kTolerance; // Notice! (what changed?)
    }

    @Override
    public void end(boolean interrupted) {
        climber.stop();
    }
}
```

`ClimberToHeightCommand` is a "Finite Command". It has a specific and articulable goal, to "move the climber to a specific height". Once the climber reaches that height the command ends.

> Pop quiz: What kind of _control loop_ is `ClimberToHeightCommand` using?
>
> - (A) PID Controller
> - (B) Bang-Bang Controller
> - (C) Proportional Controller
> - (D) Feed-Forward Controller

```java
/**
 * ClimberSetPointCommand changes the climber's height setpoint
 */
public class ClimberSetPointCommand {
    private final ClimberSubsystem climber;
    private final double setpoint;

    public ClimberSetPointCommand(ClimberSubsystem climber, double setpoint) {
        this.climber = climber;
        this.setpoint = setpoint;
        addRequirements(climber);
    }

    @Override
    public void initialize() {
        climber.setSetpoint(setpoint);
    }

    @Override
    public void execute() {
    }

    @Override
    public boolean isFinished() {
        return true; // ends immediately!?
    }

    @Override
    public void end(boolean interrupted) {
    }
}
```

`ClimberSetPointCommand` is an "Instantaneous Command". It sets the climber to a specific setpoint and then immedateily exits. Do implement this command I've had to change the assumption on how `ClimberSubsystem` is implemented. Instead of the control loop being seperate from the ClimbSubsystem (through a Command) the climber's control loop is now a local implementation detail and is assume to always be running.

This can be a decent trade-off to make, especially when:

- Your control loop is complex enough that you want to consider running it on a separate thread.
- Offloading a control loop to a motor controller often means it can run at a higher frequency (1000 Hz on a SparkMAX).
- Your subsystem only has a single mode of control.

### WPILib "New Commands"

That was a lot! I'm not sure about you, but I got tired of writing commands like this [back in 2018](https://github.com/FRC5881/2018Robot/tree/master/src/main/java/org/techvalleyhigh/frc5881/powerup/robot/commands). 30-50 lines of code can be typical for a command and we haven't even gotten to discuss complicated multistage dynamic autonomous routines! Luckily, modern versions of WPILib support something called "Command Compositions". These are powerful tools that allow us to build complex commands out of smaller parts.

> "Composition" is the same word we use in Math to describe building complex functions out of simpler parts. In this statement `h(x) = f(g(x))`, `h` is a "composition", of `f` and `g`. There's an entire field of Math and Computer science dedicated to the study of composition called [Category Theory](https://en.wikipedia.org/wiki/Category_theory).

Let's take a look at how we could rewrite `ExtendClimberCommand`, `ClimberToHeightCommand`, and `ClimberSetPointCommand` using the modern command framework:

```java
// Notice the change of reference frame - we are now writing code within ClimberSubsystem.
public class ClimberSubsystem extends Subsystem {
    // ... implementation details ommited ... //

    // The 'c' prefix is our convention for naming methods that return commands
    public Command cExtend() {
        // infinite command
        return runEnd(this::extend, this::stop);
    }

    private static final double kTolerance = 0.1;
    public Command cRunToHeight(double height) {
        // finite command
        return runEnd(() -> {
            if (getHeight() < height) {
                extend();
            } else {
                retract();
            }
        }, this::stop).until(() -> Math.abs(getHeight() - height) < kTolerance);
    }
}
```

```java
// An alternative universe where we have a control loop running in the background
public class ClimberSubsystem extends Subsystem {
    @Override
    public void periodic() {
        // Control loop goes here
    }

    public Command cSetpoint(double setpoint) {
        // instantaneous command
        return runOnce(() -> setSetpoint(setpoint));
    }
}
```

> Methods in the pattern `public Command xyz() { ... }` are called "command factories". Factories often simplify writing code.
>
> ```
> // Old - Notice you need to know the name of `ClimberSetPointCommand` to be able to write this line.
> Command goto10 = new ClimberSetPointCommand(climber, 10.0);
> // New - `cSetpoint` is much easier to find and name
> Command goto10 = climber.cSetpoint(10.0);
> ```

#### New Commands - Common Error

This next bit is the most common error made when learning the new command framework.

**Every time you call a command factory you're not yet preforming the specified action.**

Rather, you're kind of "setting up" the action to be preformed later. The `Command` is not ran until it is scheduled. Most often commands are scheduled by a `Trigger` or with the `Command.schedule()` method. Similar to how `Command`s are state-machines so is the entire "Robot" is as well.

The very first thing our code does is "setup" and create all of the `Triggers`, `Subsystems`, and `Commands` defined in `RobotContainer`.

The next phases all happen in a repeating loop (by the Command Scheduler).

1. All of the `Triggers` are checked to see if any commands need to be started or canceled.
2. All of the active `Command`s state machines are advanced.
3. All of the `Subsystems` `void periodic() { ... }` methods are called.
4. Data is sent over the network to the driver station.

> Don't rely on the exact order of these phases. It's not well defined exactly when each phase happens, just know they all happen once per tick!

### Composing Commands

Up to this point we've only talked about commands that execute a single action. It begs the question, what if we want to do multiple things at once, or in sequence? In this section we'll use an example "run the shooter, when it reaches speed feed a game piece from the indexer". This action is still a `Command` (it has a start and end) but it's more complex compared to anything we've seen thus far.

This command is certianly writable using the old framework. It requires slightly deeper

```java
public class RunShooterUntilSpeedThenRunIndexerCommand extends CommandBase {
    private final ShooterSubsystem shooter;
    private final IndexerSubsystem indexer;
    private final double speed;

    // State machine
    //
    //
    //
    private State state = State.WAIT_FOR_SPEED;
    private enum State {
        WAIT_FOR_SPEED,
        RUN_INDEXER
    }


    public RunShooterUntilSpeedThenRunIndexerCommand(ShooterSubsystem shooter, IndexerSubsystem indexer, double speed) {
        this.shooter = shooter;
        this.indexer = indexer;
        this.speed = speed;
        addRequirements(shooter, indexer);
    }

    @Override
    public void initialize() {
        this.state = State.WAIT_FOR_SPEED;
    }

    @Override
    public void execute() {
        switch (state) {
            case WAIT_FOR_SPEED -> {
                shooter.run();
                if (shooter.getSpeed() > speed) {
                    state = State.RUN_INDEXER;
                }
            }
            case RUN_INDEXER -> {
                indexer.run();
            }
        }
    }

    @Override
    public void isFinished() {
        return !indexer.hasGamePiece();
    }

    @Override
    public void end(boolean interrupted) {
        shooter.stop();
        indexer.stop();
    }
}
```

. **The complexity of writing a long-form command scales unmanageably**.

What would be better is if we could split long-form commands into the smaller parts they are clearly composed of.

- "run shooter"
- "until it reaches speed"
- and then "run indexer"

These 3 commands are all very simple

```java
public class ShooterSubsystem extends Subsystem {
    public Command cRun() {
        // infinite command
        return runEnd(this::run, this::stop);
    }

    public Command cUntilSpeed(double speed) {
        // This command has no `requirement`, defining it this way means it can run in parallel to `cRun`
        // finite command
        return Commands.waitUntil(() -> getSpeed() > speed);
    }
}

public class IndexerSubsystem extends Subsystem {
    public Command cRun() {
        // infinite command
        return runEnd(this::run, this::stop);
    }
}
```

The really magic part of the new command framework is the options we're given to compose these commands together. Here I'll give you a taste, but to learn more you're going to have to read the WPILib written documentation and JavaDocs.

```java
public class RobotContainer {
    private final ShooterSubsystem shooter = new ShooterSubsystem();
    private final IndexerSubsystem indexer = new IndexerSubsystem();
    private final CommandPS5Controller m_driverController =
        new CommandPS5Controller(OperatorConstants.kDriverControllerPort);

    public RobotContainer() {
        m_driverController.buttonCircle().whileTrue(
            Commands.runParallel(
                // run the shooter
                shooter.cRun(),
                // until it reaches speed AND THEN run the indexer
                shooter.cUntilSpeed(1000).andThen(indexer.cRun())
            )
        );
    }
}
```

Read the [WPILib documentation](https://docs.wpilib.org/en/stable/docs/software/commandbased/command-groups.html) for a deeper introduction into the most common command compositions.

Check the [`Commands` Java docs](https://github.wpilib.org/allwpilib/docs/release/java/edu/wpi/first/wpilibj2/command/Commands.html) for a complete list of factories on the `Commands` utility class.

Check the [`Command` Java docs](https://github.wpilib.org/allwpilib/docs/release/java/edu/wpi/first/wpilibj2/command/Command.html) for a complete list of decorators that can be called on a `Command`.
