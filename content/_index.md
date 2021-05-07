+++
+++

<div id="personal-intro"><div>
<img id="personal-photo" src="images/me.jpg">

# Shashank Singh

<div>

Seeking full-time employment (work visa required) in *Computer Vision, Graphics, and Software Development*

Preferred starting date: *Sep/Oct 2021*

</div>
</div></div>

<nav id="sections">
  {{ section(name="projects", active=true) }}
  {{ section(name="education") }}
  {{ section(name="work") }}
</nav>

<ul id="projects" class="section">

<h2 class="section-header">Projects</h2>

<li class="subsection resume">

## [Fission - Physically-Based Ray-Tracing Renderer](https://github.com/sshashank124/fission/)
> **Technologies**: Rust (GPU Compute, (De)Serialization, Functional Programming, SIMD, Threadpools, Parsers, Simulation), OpenEXR
>
> ***September 2019 - Present***

I have been developing a physically-based ray-tracer in Rust from scratch. This includes a custom math and graphics library, an object-loading library as well as domain-specific data-structures and algorithms.

The library contains an efficient flattened implementation of a BVH tree (to be used as an acceleration structure for complex scenes) as well as cameras, multiple BSDFs, lighting types, shapes, textures and samplers. It includes an implementation of a complete Path Tracer as well as simpler Direct Lighting and Ambient Occlusion shaders.

The entire project is structured to be modular and the code is written in a highly functional manner for easier access. Despite this, the renderer is very performant and compiles down to optimized code for all types of geometric computations.

</li><li class="subsection resume">

## [AMZ Driverless Racing](http://driverless.amzracing.ch/en/home)
> **Technologies**: C++ (ROS, Tensorflow, PCL, OpenCV), Python (Tensorflow, OpenCV), C (Atmel AVR32)
>
> ***October 2018 - August 2019***
>
> 1st place at both *Formula Student Germany* and *Formula Student East (Hungary)*

I was a member of AMZ Driverless, ETH's Formula Student Driverless team, for the 2018-2019 season. Being part of the Perception team, my primary role was to process and analyze all incoming information from the 2 cameras and 2 LiDARs on the car. My tasks involved the calibration of the various sensors in both the spatial and temporal domain to allow for fusion of the sensors data for combined processing.

My secondary role was the programming of various AVR32 microcontrollers on the car for interfacing with various components such as the Inertial Navigation System (INS), Low Voltage controller (LV), accumulators (AKKU), dashboard (DB), and the CAN interface tying them all together.

</li><li class="subsection resume">

## [EnerCage Research](https://industry.gatech.edu/technology/enercage-scalable-array-wireless-sensor-modules)
> **Technologies**: Python (OpenCV2, Tensorflow/Keras, numpy/scikit, pybluez), Java (JavaFX), C/C++ (libbluetooth, pthread, socket), Raspberry Pi, Microsoft Kinect 1 & 2
>
> ***January 2016 - December 2017***
>
> Received Warren-Batts VIP Innovation Award for significant contribution towards the team upon nomination by my research advisor.

I was a member of the EnerCage VIP Research Project at Georgia Tech under the Image Processing and Microcontrollers subteams.

Under the Image Processing subteam, I worked on training Deep Learning Convolutional Neural Network models to accurately predict and classify a lab rat's behavior from a color video and depth top-down feed. This is done in combination with various preprocessing steps to represent the data in different formats before feeding them into the training models.

Under the Microcontroller subteam, I worked with the Raspberry Pi to establish a hub between a rat-mounted microchip and a researcher's computer using Bluetooth and TCP connections respectively. I also helped design the GUI for interfacing with the Raspberry Pi Hub that would be used by the researchers on their desktops as well as implement the functionality to connect and log data from multiple hubs simultaneously.

</li><li class="subsection">

## [HyTech Racing](https://hytechracing.gatech.edu/)
### [[src]](https://github.com/hytech-racing/code-2016)
> **Technologies**: C/C++ (Arduino, Raspberry Pi, CANBUS, libbluetooth), Android (Bluetooth)
>
> ***August 2015 - June 2016***
>
> Ranked 6th in the competition from 40 universities

I was a member of the Georgia Tech HyTech Racing team where we designed a single-seat Electric Vehicle to compete against teams from up to 40 other universities at the Formula Hybrid competition and FSAE Electric.

I was responsible for linking the CANBUS network of the car and programming the various microcontrollers for functions ranging from pedal/brake control to controlling and monitoring the various EV units such as the Motor Controller, Accumulator, and Low Voltage Boards. I also installed a steering wheel-mounted smartphone that displayed gyroscope stabilized car diagnostics data such as speed, temperatures, charge, etc. via a Bluetooth connection to the car.

</li><li class="subsection">

## [Energy Jackets](https://sustain.gatech.edu/blog/turn-down-watt-greenovation-competition-winning-idea)
> **Technologies**: Python (Django, PostgreSQL), HTML/CSS, Heroku, Raspberry Pi, Analog Design (200A CT Sensors, MCP 3008)
>
> ***January 2015 - December 2017***

I served as a member of an independent, multi-disciplinary team of students who hosted our first energy savings competition in one of the on-campus freshmen housing buildings. Our project won funding from the Greenovation 2017 Fund at Georgia Tech. We designed our own monitoring system to collect live energy usage data from each of the floors in the building and log it to a remote database. We then provided statistics updated daily on the energy usage and comparative reduction for each of the floors through the competition website with the intent of
fostering constructive competition and to raise awareness with regards to energy conservation.

</li><li class="subsection">

## [Cube Companion](https://play.google.com/store/apps/details?id=com.qbix.cubecompanion)
### [[src]](https://github.com/sshashank124/cube-companion)
> **Technologies**: Android/Java (SQLite), Adobe Illustrator
>
> ***July 2015 - August 2015***

Created and published an elegant and resourceful Rubik's Cube app for Android. The app has since been download 15,000+ times and received an average rating of 4.52 stars across 234 reviews. Features of the app include:
* Timers
* Solve-Time Statistics/Management
* Beginners tutorial for 3x3
* Advanced algorithm bank
* Multiple puzzles support
* Customizable preferences

</li><li class="subsection">

## [Block Scheduler](http://block-scheduler.appspot.com/)
### [[src]](https://github.com/sshashank124/block-scheduler)
> **Technologies**: Python (Google App Engine, Google Calendar API), HTML/CSS
>
> ***December 2013 - August 2014***

Created a web app aimed at faculty and students at my highschool (ISKL) to allow for easy scheduling/unscheduling of recurring events based on our school's custom calendar which follows an 8-day cycle. Has been used by over 50 different faculty members to schedule 26,500+ events.

</li>

<h2 class="section-divider">Hackathons / Competitions</h2>

<li class="subsection">

## Google Games
> **Technologies**: Python
>
> ***Spring 2017***
>
> Received 1st place in the competition from over 20 teams

My team of five and I participated in the Google Games Competition in Atlanta which involved various programming, analytical, and puzzle solving challenges.

</li><li class="subsection">

## Freshmen Hackathon
> **Technologies**: Python
>
> ***Spring 2015***
>
> Ranked 3rd in the competition from over 100 participants

I participated in the Freshmen Hackathon which tested problem solving skills using programming.

</li><li class="subsection">

## Shake2Meet
> **Technologies**: Android (Bluetooth, Firebase), Myo Gesture Control Armband Android SDK
>
> ***September 2015***

For our HackGT 2015 project, my team of four designed an Android App that synced with an arm-worn smart band with 3-axis accelerometer sensors. This app would be installed and two or more individuals devices. Based on the accelerometer data and time, it would recognize when two individuals were shaking hands and would automatically exchange their contact information (with prior consent) via a Firebase database.

</li>

</ul>

<ul id="education" class="section">

<h2 class="section-header">Education</h2>

<li class="subsection resume">
<details>
<summary>

## [ETH Zurich](https://ethz.ch/en/the-eth-zurich.html)
### ([Computer Science MSc.](https://inf.ethz.ch/studies/master/master-cs.html))

> ***September 2018 - Present***

</summary>

### **Courses**
* Computer Graphics
* Computer Vision
* Physically-Based Simulation
* Advanced Machine Learning
* Design of Parallel and High-Performance Computing
* Computational Intelligence Lab
* Algorithms Lab
* System Security
* Deep Learning for Computer Vision: Seminal Work
* A Sampler of Histories and Philosophies of Mathematics

</details>

</li><li class="subsection resume">

<details>
<summary>

## [Georgia Institute of Technology](https://www.gatech.edu/about)
### ([Computer Engineering BSc.](https://www.ece.gatech.edu/computer-engineering-degree)), Atlanta

> **Minor**: Computer Science  
> **GPA**: 3.76
>
> ***August 2014 - December 2017***
>
> Graduated with **Highest Honors**

</summary>

### **Fall 2017**
* Senior Design 2 (*ECE 4012*)
* Advanced Computer Architecture (*ECE 4100*)
* Introduction to Network Security (*ECE 4894*)

### **Spring 2017**
* Game AI (*CS 4731*)
* Introduction to Perception and Robotics (*CS 3630*)
* Senior Design 1 (*ECE 4011*)
* Introduction to International Relations (*INTA 1110*)

### **Fall 2016**
* Computer Organization and Programming (*CS 2110*)
* Introduction to Artificial Intelligence (*CS 3600*)
* ECE Professional/Technical Communications (*ECE 3005*)
* Advanced Programming Techniques (*ECE 4122*)
* General Psychology (*PSYC 1101*)

### **Summer 2016**
* Computer Communications (*ECE 3600*)
* Machine Learning (*CS 4641*)
* Architecture, Concurrency, and Energy in Computation (*ECE 3056*)
* Statics (*COE 2001*)
* Principles and Applications of Engineering Materials (*MSE 2001*)

### **Spring 2016**
* Physical Foundations of Computer Engineering (*ECE 3030*)
* Data Structures and Algorithms (*CS 1332*)
* French Culture 1 (*FREN 2001*)
* Introduction to Modern Physics (*PHYS 2213*)

### **Fall 2015**
* Mathematical Foundations of Computer Engineering (*ECE 3020*)
* Introduction to Object-Oriented Programming (*CS 1331*)
* Circuit Analysis (*ECE 2040*)
* Differential Equations (*MATH 2552*)
* Government of the U.S. (*POL 1101*)
* Probability and Statistics with Applications (*MATH 3670*)
* Digital Design Laboratory (*ECE 2031*)

### **Spring 2015**
* Introduction to Signal Processing (*ECE 2026*)
* Programming for Hardware/Software Systems (*ECE 2035*)
* Engineering Software Design (*ECE 2036*)
* Calculus III (*MATH 2401*)
* Hardware Security Team - Vertically Integrated Program (*ECE 2811*)
* Foundations of Health (*APPH 1040*)

### **Fall 2014**
* Computing for Engineers (*CS 1371*)
* Digital System Design (*ECE 2020*)
* Principles of Microeconomics (*ECON 2106*)
* English Composition II (*ENGL 1102*)
* GT Freshman Seminar (*GT 1000*)
* Calculus II (*MATH 1502*)

</details>

</li><li class="subsection">

## [International School of Kuala Lumpur](https://www.iskl.edu.my)


> **IB Diploma Score**: 42  
> **GPA**: 3.87
>
> ***August 2010 - May 2014***
>
> Received Book Award for Mathematics (given to one student in the whole High School as chosen by a committee from the HS Math Department)

</li>

</ul>

<ul id="work" class="section">

<h2 class="section-header">Work</h2>

<li class="subsection resume">

## [Android Development Intern (R&D) at VMware AirWatch](https://www.air-watch.com/)
> **Technologies**: Android (Device OEM APIs), Stash, JIRA, Bamboo, Confluence, Agile
>
> ***May 2017 - August 2017***
>
> ***Atlanta***

I interned at AirWatch on the Android Agent App Development team. My primary project involved refactoring and restructuring existing code with regards to device application policy control/management/restricition. I learned new team-based technologies such as Stash for codebases, JIRA for issue tracking, Bamboo for build management, and Confluence for documentation.

</li><li class="subsection">

## Teaching Assistant for Data Structures and Algorithms (CS 1332) and Object-Oriented Programming (CS 1331)
> **Technologies**: Java (1.8, JavaFX, JUnit4)
>
> ***January 2016 - May 2017***
>
> ***Georgia Tech, Atlanta***

I was a TA for the two CS courses. My responsibilities included writing homework assignments, preparing for and conduction weekly recitations, grading homeworks/tests, and holding office hours.

</li><li class="subsection">

## Tutor for Digital Design (ECE 2020) and Programming Hardware/Software Systems (ECE 2035, 2036)
> **Technologies**: C/C++ (MIPS ISA)
>
> ***January 2016 - May 2017***
>
> ***Georgia Tech, Atlanta***

Tutored multiple ECE courses and aided students with lab assignments and grading. Course materials included:
* MIPS ISA
* C memory management
* C++ STL
* Digital Design Logic

</li>

</ul>
