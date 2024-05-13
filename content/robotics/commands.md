---
page-title: Abstraction
order: 2
---

# Command Based Programming

"Command Based Programming" is the design pattern we use to structure our robot code. We use abstraction to separate our robot into modular chunks.

I like to preface this section by reminding you that robots are complex systems, built from very dumb parts. The "robot" is not at all intelligent, it's just a collection of sensors and actuators that are themselves mostly made of metal and plastic. The most fundamental control of our robot is setting voltages and reading voltages. Everything else is just a layer of abstraction on top of voltages.

> Read more about abstraction in Computer Science: <https://en.wikipedia.org/wiki/Abstraction_(computer_science)>.

## Subsystems

The first step to programming a robot is identifying in what way we should group up sensors and actuators into "Subsystems". In the 2023-2024 season our robot had 14 motors, 4 absolute encoders, 1 gyro/imu, 1 proximity IR sensor, and an LED controller. We grouped these into 7 subsystems:

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

Notice how each actuator and sensor is only in one subsystem. This is a _requirement_ of the Command Based Programming framework. If two subsystems required the same motor, then we would have to deal with the two systems fighting with each other. Imagine 1 subsystem trying to drive forward while the other tries to drive backwards. That could cause very erratic and dangerous behavior.

Separating the robot into subsystems is a way to enforce modularity. Each subsystem is responsible for controlling its own actuators and reading its own sensors. This makes it easier to reason about the robot's behavior and to test each subsystem in isolation. During build season modularity is critical because we're often only going to have access to partially assembled robots. Being able to test our climber without our shooter gives us more opportunities to test at all. Potentially we can find design issues earlier in the season and adjust.

### Private vs Public Methods

The biggest constraint in FRC is time. Especially on programming team, we won't always have access to the robot to test our code. Robots are constantly getting disassembled and reassembled, and generally "building the robot" is a higher priority than "programming the robot". Build team has a habit of making near constant design changes to the robot. One day we'll have a climber with 1 motor and a limit switch and the next it will be 2 motors and a laser rangefinder, or a shooter with 1 flywheel to a shooter with 2. Programming team is working at our best when we're able to respond to those changes quickly.

Fundamentally, changing from a 1-motor climber to a 2-motor climber doesn't really change the robot's intended behavior. No matter how many motors are on the climber we still expect it to _move up_ and _move down_. A climber could be built with a winch, a scissor lift, a telescoping arm, or a pneumatic piston and it would still _move up_ and _move down_. A shooter that has 1 wheel or 2 wheels still _shoots_. A ground intake that uses CTRE Falcon motors or REV Neo motors still _intakes_. Hopefully you can see the pattern here, no matter how we "implement" a subsystem they typically have the same "intended behavior".

As you become stronger programmers you find your focus shifts from low level implementation details to high level behavior. Great software engineering teams hardly ever talk about the specifics of what lines of code to write. We spend less time talking about whether you should use a `switch` statement or an `if` statement and more time talking about requirements and design.

Writing a subsystem generally turns into a 2 step process:

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
    // Notice: `private` means this is an implementation detail
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
 * RobotContainer is where we use subsystems and commands. It's where we connect
 * controllers to subsystems.
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

Here we have a very simple robot, it only has a climber that extends and retracts. It does this by running the climber at either 50%, -50% (reverse), and then 0% (stopping) when nothing is commanded.

Let's throw a wrench in this design, build team has decided they want to add a second motor to the climber axle. Adding a motor on the opposite side of the climber is an easy way to increase a mechanism's power.

Here's how we would change the code:

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
        motor.set(0.5);
+       motor2.set(-0.5);
    }

    private retract() {
        motor.set(-0.5);
+       motor2.set(0.5);
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
 * RobotContainer is where we use subsystems and commands. It's where we connect
 * controllers to subsystems.
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

Notice how we didn't have to change the `RobotContainer` at all. The `RobotContainer` class only knows about the `ClimberSubsystem`'s public methods, and therefore it just _doesn't care_ how the climber is implemented. It only cares that the climber can extend and retract. This is climber's _contract_ and it's an example of abstraction.

## Commands

If "subsystems" represent the _nouns_ of our robot then "commands" represent the _verbs_. If you can say "the robot has a climber" then that implies `ClimberSubsystem` should exist. Saying "the climber can extend and retract" implies the existence of 2 `Command`s, "climber extend" and "climber retract". WPILib commands have a lot of depth, they're designed to both represent simple single action commands like `LedSubsystem::setColor(Color c)` to larger composable actions like `ShooterSubsystem::runAfterReachingSpeed(Velocity v, Command c)` up to award winning autonomous routines. All of those actions are commands.

### Scheduling

Commands are ran by the "Command Scheduler". By default the scheduler runs every 20ms (50 Hz) and each "tick" running commands are processed. The scheduler enforces what I call the "law of commands", which states "at all times each `Subsystem` can only be required by at most 1 `Command`". This is a safety feature to prevent the robot from doing dangerous things like trying to drive forward and backwards at the same time.

In order to enforce this law Commands must accurately communicate their subsystem requirements to the scheduler. If 2 commands require the same subsystem then the scheduler will "cancel" one of the commands.

When dealing with any kind of "scheduler" we often will make Gantt Charts to help visualize timelines. On these charts the horizontal axis represents time and each subsystem gets a unique row. Commands are represented as blocks of time where the subsystem is required.

Here's a simple example:

![Single Requirement Gantt Chart](/static/img/gantt.png)

Notice how none of the Red blocks overlap, this is because of the "law of commands". Overlapping commands like this are forbidden.

![Invalid Schedule](/static/img/gantt-1.png)

Here we have 2 conflicts, commands D and E conflict and commands C and F conflict. When presented with this situation the scheduler has to make a choice about which command to run. This choice is defined by the first command's [interrupt behavior](https://github.wpilib.org/allwpilib/docs/release/java/edu/wpi/first/wpilibj2/command/Command.InterruptionBehavior.html)

By default the scheduler will interrupt the existing command and start running the new command `kCancelSelf`. Important commands can be marked as `kCancelIncoming` which will ignore the new command and continue running the existing command. `kCancelIncoming` should only be used rarely because of how it "eats" robot controls. The following charts shows the scheduler's behavior when faced with a conflict:

![kCancelSelf](/static/img/gantt-2.png)

![kCancelIncoming](/static/img/gantt-3.png)

#### Miscellaneous Notes

Commands requiring multiple subsystems are allowed, they don't violate the "law of commands". F and G in the above charts are examples of this.

Commands don't necessarily need to require a subsystem at all. We'll often use this feature when we want to run a "waiting" command. `Commands.waitSeconds(double t)` is one example of a command that doesn't require a subsystem.

### Lifecycle of a `Command`

`Command`s are a state machine representing a complete action to be performed by the robot. The state machine is built from 4 states: `initialize`, `execute`, `isFinished`, and `end`.

1. `void initialize()` - Called once when the command is started
2. `void execute()` - Called every tick while the command is running
3. `boolean isFinished()` - Called after `execute()`, returns `true` when the command should end
4. `void end(boolean interrupted)` - Called once when the command is ended. `interrupted` is `true` if the command was canceled by the scheduler, `false` if the command ended normally because `isFinished()` returned `true`.

![alt text](/static/img/gantt-4.png)

Back in the day _all_ commands were written by explicitly writing out these methods. Sometimes it might still be necessary to write a command this way, but today WPILib has a lot of helper classes to make writing commands easier.

#### Long-form Command examples

There are 3 common patterns when writing long-form commands:

- Infinite Commands
- Finite Commands
- Instantaneous Commands

Here's an example of each:

```java
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
        return false; // never ends on it's own
    }

    @Override
    public void end(boolean interrupted) {
        climber.stop();
    }
}
```

`ExtendClimberCommand` here is an "Infinite Command". It runs forever, unless if something else interrupts it.

```java
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
        return Math.abs(climber.getHeight() - height) < kTolerance; // ends when a condition is met
    }

    @Override
    public void end(boolean interrupted) {
        climber.stop();
    }
}
```

`ClimberToHeightCommand` is a "Finite Command". It has a specific goal, to move the climber to a specific height. Once the climber reaches that height the command ends.

> Pop quiz: What kind of _control loop_ is `ClimberToHeightCommand` using?
>
> - (A) PID Controller
> - (B) Bang-Bang Controller
> - (C) Proportional Controller
> - (D) Feed-Forward Controller

```java
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
        return true; // ends immediately
    }

    @Override
    public void end(boolean interrupted) {
    }
}
```

`ClimberSetPointCommand` is an "Instantaneous Command". It sets the climber to a specific setpoint and then ends. Here we're assuming that `ClimberSubsystem` is running the control loop in the background.

### WPILib "New Commands"

I'm not sure about you, but I got tired of writing commands like this [back in 2018](https://github.com/FRC5881/2018Robot/tree/master/src/main/java/org/techvalleyhigh/frc5881/powerup/robot/commands), there's a better way. WPILib supports something called "Command Compositions". These are powerful tools that allow us to build complex commands out of smaller parts. Let's take a look at how we could rewrite `ExtendClimberCommand`, `ClimberToHeightCommand`, and `ClimberSetPointCommand` using the new command framework:

```java
public class ClimberSubsystem extends Subsystem {
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
        // Run control loop here
    }

    public Command cSetpoint(double setpoint) {
        // instantaneous command
        return runOnce(() -> setSetpoint(setpoint));
    }
}
```

New commands use a patterned called "declarative programming" as opposed to old command's "imperative programming". Both styles are capable of doing the same things, but it requires a different mindset for each. They also fit different niches, you'll see we'll often use the old command style for our driver controls because they're easier to step-through.

### Composing Commands

Up to this point we've only talked about commands that complete a single action. But what if we want to do multiple things at once? Think "drive forward while spinning the intake" or "run the shooter until it reaches speed then run the indexer". These actions are still 'commands' (they have a start and end) but they're composed of multiple smaller commands.

The list of available compositions is long, too long for this page. Here are 3 sources to get you started:

Check the [WPILib documentation](https://docs.wpilib.org/en/stable/docs/software/commandbased/command-groups.html) for an introduction.

Check the [`Commands` Java docs](https://github.wpilib.org/allwpilib/docs/release/java/edu/wpi/first/wpilibj2/command/Commands.html) for a complete list of factories on the `Commands` class.

Check the [`Command` Java docs](https://github.wpilib.org/allwpilib/docs/release/java/edu/wpi/first/wpilibj2/command/Command.html) for a complete list of decorators that can be called on a `Command`.
