# Thread Pool

You have to be very careful when creating async tasks that you don't end up performing a synchronous function that blocks the thread, as then ultimately the reactor can never receive the message to drain the channel, resulting in your entire app being blocked. As a result, it's best practice to be very aware of what kind of task you're doing (IO bound (have to wait for lots of data) or CPU bound (need to keep things in-mem and perform calculations etc) - just need to be aware of this). 

Generally speaking, if you want to perform IO in an async manner, you should spawn threads to do it away from the main thread and avoid blocking.