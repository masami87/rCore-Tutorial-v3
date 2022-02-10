# Note

注意`trap.S`中的__alltraps和__restore方法用来陷入trap和恢复。
而`switch.S`用来做Task的切换。
任务切换是第二章提及的 Trap 控制流切换之外的另一种异常控制流，都是描述两条控制流之间的切换，如果将它和 Trap 切换进行比较，会有如下异同：

    与 Trap 切换不同，它不涉及特权级切换；

    与 Trap 切换不同，它的一部分是由编译器帮忙完成的；

    与 Trap 切换相同，它对应用是透明的。

任务切换必须是内核来做，因此就是当要切换任务时执行trap_handler，然后调用相关的内核功能`suspend_current_and_run_next`，
该函数中调用了switch。

TrapContext中保存的寄存器记录了应用陷入S特权级之前的CPU状态，而TaskContext则可以看成一个应用在S特权级进行Trap处理的过程中调用__switch之前的CPU状态。当恢复TaskContext之后会继续进行Trap处理，而__restore恢复TrapContext之后则是会回到用户态执行应用。
