---
hidden: true
---

# Control Theory

In [Abstraction](../abstraction.md) we learned how to structure a Robot Program, in this section we will learn how to successfully control a robot. Control Theory is the study of how to control a system, in our case a robot. Some of this section is based on the math presented in Tyler Veness' [Controls Engineering in the FIRST Robotics Competition](file:///home/mahonec/Downloads/controls-engineering-in-frc.pdf) book, while for the most part the formulation of ideas are my own.

Students with any experience in robotics will be familar with misbehaving robots. Robot mechanisms can be difficult to control and often fall into oscillations or fail to perform the desired task. The hardest, and most common, mechanisms to control use DC motors. For example, in the 2024 Cresendo season our robot used 14 motors and 0 alterative control mechanisms (pneumatics, servos, etc.).

## The Big 3 DC Motor Mechanisms

In FRC, the 3 most common DC motor mechanisms are:

1. Wheels
2. Elevators
3. Arms

![White board diagrams](/static/img/big_three.jpg)

Most mechanisms are either exactly one of these, or a combination of them. For example, an intake might be a combination of an arm and wheels, a launcher might use wheels as a turrent and as a flywheel. What sets these mechanisms apart are their dynamics. How they move, where they are controlled from, and the forces they experience.

### Wheels

Wheels are the principle component of drivetrains, launchers, and most intakes. Other than our Climber our 2024 Cresendo robot was _only_ wheels. Wheel's fit into 2 control categories, velocity controlled (flywheels) or position controlled (turrets). A swerve module provides an example of both.

![SDS Mk4 Swerve Module CAD](/static/img/sds_cad.jpg)

Swerve modules have 1 motor for 'steering' and another for 'driving'. The goal of controlling the steering motor is to precisely, and quickly, move the module to a desired angle (sometimes called azimuth). While the driving motor is controlled to spin at a desired velocity. You see the same velocity-only control in launchers and intakes - when launching a game piece you don't really need to know if the wheel has made 100 or 1000 full rotations, we just need to know if it's spinning at the desired RPM.

### Elevators

Elevators go up and down and they are the first mechanism that may experience gravity. While controlling elevators we are primarily concerned with their position. In my diagram of an elevator I showed a basic winch-and-pulley system but my discussion of elevators covers most linear mechanisms (scissor lifts, telescoping tubes, etc.).
