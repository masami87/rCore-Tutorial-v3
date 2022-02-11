# Note

## 一直到ch5的内容
注意`trap.S`中的__alltraps和__restore方法用来陷入trap和恢复。
而`switch.S`用来做Task的切换。
任务切换是第二章提及的 Trap 控制流切换之外的另一种异常控制流，都是描述两条控制流之间的切换，如果将它和 Trap 切换进行比较，会有如下异同：

    与 Trap 切换不同，它不涉及特权级切换；

    与 Trap 切换不同，它的一部分是由编译器帮忙完成的；

    与 Trap 切换相同，它对应用是透明的。

任务切换必须是内核来做，因此就是当要切换任务时执行trap_handler，然后调用相关的内核功能`suspend_current_and_run_next`，
该函数中调用了switch。

TrapContext中保存的寄存器记录了应用陷入S特权级之前的CPU状态，而TaskContext则可以看成一个应用在S特权级进行Trap处理的过程中调用__switch之前的CPU状态。当恢复TaskContext之后会继续进行Trap处理，而__restore恢复TrapContext之后则是会回到用户态执行应用。
另外，保存TrapContext之后进行Trap处理的时候，s0-s11寄存器可能会被覆盖，后面进行任务切换时这些寄存器会被保存到TaskContext中，也就是说这两个Context中的s0-s11也很可能是不同的。


当实现了将TrapContext存入用户空间的次高地址后，每次陷入trap时寄存器sscratch 存放的就是当前用户空间中的
TrapContext的地址，这个放置的动作是在__restore回该Trap流时完成的(main分支中的trap.S有注释)。

实现任务的切换就是陷入trap，然后内核调用__switch从而切换到另一个TaskContext，而上面说了，TaskContext是在进行Trap处理
的过程中调用__switch之前的cpu的状态，因此切换完之后cpu还在S特权级，使用的栈是该TaskContext的内核栈。接下来trap_handler会
跳到trap_return，这里会获取切换后的TrapContext，并将它的地址作为参数放入a0中，接下来trap_return执行__restore，在__restore
中会取这个地址，然后根据TrapContext的内容恢复寄存器状态，TrapContext中包含了spec寄存器，即sret之后要指令的地址，当trap结束
后会跳到这个地址继续执行。
