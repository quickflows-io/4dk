# 4dk-core
<em>(Domain Driven Design Development Kit)</em>

## Requirements
OpenJDK 17 <br/>
maven 3.8.3 <br/>

No additional library are used in this project. <br/>
Clone the project. Enter to `/project/core` folder and execute following command `mvn install`. <br /> 

## Presentation
This project proposes a framework to easily transpose an "Event Storming" into your code. <br/>
This framework is based on famous concept such as CQRS, Hexagonal Architecture, CommandBus pattern. <br />
If you have never heard about "Domain Driven Design" or "Event Storming", you have to discover it. It will change your vision of software development.

## Objectives

Event storming is a tool to help you to identify your tool. One of the event storming is to help you to design your domains.<br/>
At the end of your "event storming", the result will look such as something like this: </br>

<img src="https://i2.wp.com/cleandojo.com/wp-content/uploads/2019/06/Bounded-Context-1050x383.png?ssl=1" style="float: left; margin-right: 10px;" />

The next step is from this to develop your software.<br/>

Event Storming design the behaviours of your domains with commands and events. <br/>

These framework implements with bus all the successions rules between event storming "post it" :<br/>

### Command (Blue post-it): 
`org.tby.fourdk.core.command.Command` class matches this post it. <br />
A Command is handled by a unique commandhandler (`org.tby.fourdk.core.command.CommandHandler`). <br />
A CommandHandler is a <a href="http://www.plainionist.net/Implementing-Clean-Architecture-UseCases/">Clean Architecture Usecase</a> which can only be triggered by its associated command.<br/>
It returned a list of `Event` generated by the domain transaction.


### Event (Orange post-it)
`org.tby.fourdk.core.event.Event` class matches this post it. <br />
An event is handled by one or several eventhandler (`org.tby.fourdk.core.event.EventHandler`). <br />
An EventHandler is a <a href="http://www.plainionist.net/Implementing-Clean-Architecture-UseCases/">Clean Architecture Usecase</a> which can only be triggered by its associated command.


### Policy (Purple post-it)
`org.tby.fourdk.core.event.EventHandler` class could match this post it. <br />
More exactly, it will be the <a href="http://www.plainionist.net/Implementing-Clean-Architecture-UseCases/">Clean Architecture Usecase</a> which will trigger the policy rule from a specific event. 
