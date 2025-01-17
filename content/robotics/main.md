---
hidden: true
---

```java
public class ShooterSubsystem extends Subsystem {
    private void run() { /* run motor at 100% */ }
    private void stop() { /* stops the motor */ }
    private double getSpeed() { /* returns the speed of the motor */ }

    public Command cRun() {
        return runEnd(this::run, this::stop);
    }

    public Command cWaitUntilSpeed(double rpm) {
        return Commands.waitUntil(() -> getSpeed() >= rpm);
    }
}

public class IndexerSubsystem extends Subsystem {
    private void run() { /* run motor at 100% */ }
    private void stop() { /* stops the motor */ }

    public Command cRun() {
        return runEnd(this::run, this::stop);
    }
}

public class RobotContainer {
    private final ShooterSubsystem shooter = new ShooterSubsystem();
    private final IndexerSubsystem indexer = new IndexerSubsystem();
    private final CommandPS5Controller m_driverController =
        new CommandPS5Controller(OperatorConstants.kDriverControllerPort);

    public RobotContainer() {
        m_driverController.buttonCircle().whileTrue(
            Commands.runParallel(
                shooter.cRun(),
                shooter.cWaitUntilSpeed(1000).andThen(indexer.cRun())
            )
        );
    }
}
```
