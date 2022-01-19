# 4dk
<em>(Domain Driven Design Development Kit)</em>

## Presentation
This project proposes a framework to easily transpose an "Event Storming" into your code. <br/>
This framework is based on famous concept such as CQRS, Hexagonal Architecture, CommandBus pattern. <br />
If you have never heard about "Domain Driven Design" or "Event Storming", you have to discover it. It will change your vision of software development.<br/>
This projects presents an implementation in <b>Java</b> and another in <b>Rust</b>

## Objectives

Event storming is a tool to help you to identify your core business. One of the event storming goal is to help you to design your domains.<br/>
At the end of your "event storming", the result will look such as something like this: </br>

<img src="https://i2.wp.com/cleandojo.com/wp-content/uploads/2019/06/Bounded-Context-1050x383.png?ssl=1" style="float: left; margin-right: 10px;" />

The next step is to transpose it into your code.<br/>

Event Storming design the behaviours of your domains with commands and events. <br/>

These framework implements with bus all the successions rules between event storming "post it" :<br/>

To see "how to switch from events storming into the code" please follow java samples. You will see how an "events storming" could be your software design.

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


## Features

### Command
| Features                                        | Description                                                                                 | Java | Rust |
|-------------------------------------------------|---------------------------------------------------------------------------------------------|------|------|
| CommandDispatcher                               | CommandBus which dispatches command to its associated CommandHandler                        | X    | X    |
| CommandLoggingMiddleware                        | CommandBus which logs all command handled and events resulted by the transactionwhich       | X    |      |
| EventsProducedByCommandBusPersistenceMiddleware | CommandBus which saves all events resulted by the transaction in an event store             | X    |      |
| EventsProducedByCommandBusDispatcher            | CommandBus which dispatches resulted events to an EventBus                                  | X    | X    |
| SecuredCommandDispatcher                        | CommandDispatcher which dispatches SecuredCommands only if they have correct authorization  | X    | ...  |

### Query
| Features               | Description                                                                            | Java | Rust |
|------------------------|----------------------------------------------------------------------------------------|------|------|
| QueryDispatcher        | QueryBus which dispatches query to its associated QueryHandler                         | X    | X    |
| QueryLoggingMiddleware | QueryBus which logs all query handled and responses resulted                           | X    |      |
| SecuredQueryDispatcher | QueryDispatcher which dispatches SecuredQuery only if they have correct authorization  | X    | ...  |

### Event
| Features        | Description                                                     | Java | Rust |
|-----------------|-----------------------------------------------------------------|------|------|
| EventDispatcher | EventBus which dispatches event to its associated EventHandlers | X    | X    |