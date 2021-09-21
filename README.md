# MVS 3.8J job Submit Tool Written In Rust

#### Introduction

MVS 3.8J has an external card reader that can be used to remotely submit jobs to MVS.

It supplies a port usually 3505 by default that a socket reader is listening to. You can connect to that TCP port via a socket client and send a stream of ascii bytes (jcl file). The socket reader will read the stream of bytes, convert it to EBCIDIC and submits a job to the *Internal Reader*.

#### Vocabulary 

Lets go over some terms that you will need to understand about running jobs on MVS.

1. **JOB** - Running a batch job on MVS. Usually a program or mainframe utility.

2. **JCL** - Job Control Language (JCL) is a name for scripting languages used on MVS 3.6J to instruct the system on how to run a batch job or start a subsystem. See the link below:  

   []: https://en.wikipedia.org/wiki/Job_Control_Language

   

3. **Card Reader** - On MVS, it emulates the old mainframe card readers that read a deck of physical cards. MVS supplies a card reader on port 3505 that we can connect to via a TCP socket.

   []: https://en.wikipedia.org/wiki/Punched_card_input/output

4. **HERCULES Console** - The console that is started when you start MVS on you system.

5. **MVS 3.8J** - Operating system that runs on top of HERCULES mainframe emulator. 

   []: http://wotho.ethz.ch/tk4-/

6. 