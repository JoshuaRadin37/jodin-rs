 pushq   %r14
 pushq   %rsi
 pushq   %rdi
 pushq   %rbx
 movl    $118232, %eax
 callq   ___chkstk_ms
 subq    %rax, %rsp
 movq    %r9, 2784(%rsp)
 movq    %r8, 2800(%rsp)
 movq    %rdx, 2792(%rsp)
 movq    %rcx, %rax
 movq    2800(%rsp), %rcx
 movq    %rax, 2808(%rsp)
 movq    %rax, 2816(%rsp)
 movq    %rdx, 118008(%rsp)
 movb    $0, 118007(%rsp)
 movb    $0, 117930(%rsp)
 movb    $0, 117912(%rsp)
 movb    $0, 117949(%rsp)
 movb    $0, 117937(%rsp)
 movb    $0, 117925(%rsp)
 movb    $0, 117932(%rsp)
 movb    $0, 117950(%rsp)
 movb    $0, 117988(%rsp)
 movb    $0, 117914(%rsp)
 movb    $0, 117911(%rsp)
 movb    $0, 117944(%rsp)
 movb    $0, 117933(%rsp)
 movb    $0, 117982(%rsp)
 movb    $0, 117998(%rsp)
 movb    $0, 117928(%rsp)
 movb    $0, 117977(%rsp)
 movb    $0, 117994(%rsp)
 movb    $0, 117955(%rsp)
 movb    $0, 117942(%rsp)
 movb    $0, 117939(%rsp)
 movb    $0, 117992(%rsp)
 movb    $0, 117966(%rsp)
 movb    $0, 117985(%rsp)
 movb    $0, 117967(%rsp)
 movb    $0, 117915(%rsp)
 movb    $0, 117968(%rsp)
 movb    $0, 117953(%rsp)
 movb    $0, 117969(%rsp)
 movb    $0, 117952(%rsp)
 movb    $0, 117959(%rsp)
 movb    $0, 117978(%rsp)
 movb    $0, 117913(%rsp)
 movb    $0, 117995(%rsp)
 movb    $0, 117921(%rsp)
 movb    $0, 117979(%rsp)
 movb    $0, 117997(%rsp)
 movb    $0, 117956(%rsp)
 movb    $0, 117946(%rsp)
 movb    $0, 117918(%rsp)
 movb    $0, 117938(%rsp)
 movb    $0, 117926(%rsp)
 movb    $0, 118002(%rsp)
 movb    $0, 118005(%rsp)
 movb    $0, 117922(%rsp)
 movb    $0, 117941(%rsp)
 movb    $0, 117924(%rsp)
 movb    $0, 117936(%rsp)
 movb    $0, 117919(%rsp)
 movb    $0, 117983(%rsp)
 movb    $0, 118003(%rsp)
 movb    $0, 117945(%rsp)
 movb    $0, 117934(%rsp)
 movb    $0, 117999(%rsp)
 movb    $0, 117923(%rsp)
 movb    $0, 117989(%rsp)
 movb    $0, 117943(%rsp)
 movb    $0, 117948(%rsp)
 movb    $0, 117970(%rsp)
 movb    $0, 117920(%rsp)
 movb    $0, 117951(%rsp)
 movb    $0, 118006(%rsp)
 movb    $0, 117971(%rsp)
 movb    $0, 117986(%rsp)
 movb    $0, 117974(%rsp)
 movb    $0, 117987(%rsp)
 movb    $0, 117935(%rsp)
 movb    $0, 117917(%rsp)
 movb    $0, 117976(%rsp)
 movb    $0, 117940(%rsp)
 movb    $0, 117929(%rsp)
 movb    $0, 117975(%rsp)
 movb    $0, 117960(%rsp)
 movb    $0, 117931(%rsp)
 movb    $0, 117954(%rsp)
 movb    $0, 117962(%rsp)
 movb    $0, 117958(%rsp)
 movb    $0, 118004(%rsp)
 movb    $0, 117957(%rsp)
 movb    $0, 117961(%rsp)
 movb    $0, 117981(%rsp)
 movb    $0, 117972(%rsp)
 movb    $0, 117947(%rsp)
 movb    $0, 117927(%rsp)
 movb    $0, 117963(%rsp)
 movb    $0, 117973(%rsp)
 movb    $0, 117964(%rsp)
 movb    $0, 117990(%rsp)
 movb    $0, 117984(%rsp)
 movb    $0, 117965(%rsp)
 movb    $0, 117996(%rsp)
 movb    $0, 118000(%rsp)
 movb    $0, 117991(%rsp)
 movb    $0, 117993(%rsp)
 movb    $0, 117980(%rsp)
 movb    $0, 117916(%rsp)
 movb    $0, 118001(%rsp)
 movb    $1, 117914(%rsp)
 callq   jodin_rs::ast::pair_as_rules
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2824(%rsp)
 movq    %rcx, 2832(%rsp)
 jmp     .LBB7390_1
.LBB7390_1:
 movq    2800(%rsp), %rcx
 movq    2824(%rsp), %rax
 movq    2832(%rsp), %r8
 movq    %r8, 2840(%rsp)
 movq    %rax, 2848(%rsp)
 movb    $1, 117913(%rsp)
 movq    %rcx, %rax
 movq    %rax, 2768(%rsp)
 callq   core::mem::size_of_val
 movq    %rax, %rcx
 movq    %rcx, 2776(%rsp)
 jmp     .LBB7390_4
.LBB7390_2:
 movq    2784(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_1661
.LBB7390_3:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_2
.LBB7390_4:
 movq    2776(%rsp), %rcx
 movq    2768(%rsp), %rax
 addq    %rcx, %rax
 movq    %rax, 2760(%rsp)
 setb    %al
 testb   $1, %al
 jne     .LBB7390_8
 jmp     .LBB7390_7
.LBB7390_5:
 testb   $1, 117913(%rsp)
 jne     .LBB7390_1689
 jmp     .LBB7390_2
.LBB7390_6:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_5
.LBB7390_7:
 movq    2800(%rsp), %rdx
 movq    2792(%rsp), %rcx
 movq    2760(%rsp), %rax
 movq    %rax, 118048(%rsp)
 callq   jodin_rs::ast::JodinNodeGenerator::stack_size
 movq    %rax, %rcx
 movq    %rcx, 2752(%rsp)
 jmp     .LBB7390_10
.LBB7390_8:
 leaq    str.0(%rip), %rcx
 leaq    .L__unnamed_358(%rip), %r8
 movl    $28, %edx
 callq   core::panicking::panic
 jmp     .LBB7390_9
.LBB7390_9:
 ud2
.LBB7390_10:
 movq    2752(%rsp), %rcx
 callq   jodin_rs::utility::Bytes::new
 movq    %rax, %rcx
 movq    %rcx, 2744(%rsp)
 jmp     .LBB7390_11
.LBB7390_11:
 movq    2792(%rsp), %rcx
 movq    2744(%rsp), %rax
 movq    %rax, 2856(%rsp)
 addq    $24, %rcx
 callq   <alloc::vec::Vec<T,A> as core::ops::deref::Deref>::deref
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2728(%rsp)
 movq    %rcx, 2736(%rsp)
 jmp     .LBB7390_12
.LBB7390_12:
 movq    2728(%rsp), %rdx
 movq    2736(%rsp), %rcx
 callq   core::slice::<impl [T]>::last
 movq    %rax, %rcx
 movq    %rcx, 2720(%rsp)
 jmp     .LBB7390_13
.LBB7390_13:
 movq    2720(%rsp), %rax
 movq    %rax, 2864(%rsp)
 movl    $1, %eax
 xorl    %ecx, %ecx
 cmpq    $0, 2864(%rsp)
 cmoveq  %rcx, %rax
 cmpq    $1, %rax
 jne     .LBB7390_15
 movq    2760(%rsp), %rcx
 movq    2864(%rsp), %rax
 movq    (%rax), %rax
 movq    %rax, 118088(%rsp)
 subq    %rcx, %rax
 movq    %rax, 2712(%rsp)
 seto    %al
 testb   $1, %al
 jne     .LBB7390_29
 jmp     .LBB7390_28
.LBB7390_15:
 movq    2800(%rsp), %rcx
 movq    %rcx, 3256(%rsp)
 leaq    3264(%rsp), %rcx
 leaq    2856(%rsp), %rdx
 callq   <jodin_rs::utility::Bytes as jodin_rs::utility::HumanReadable>::human_readable
 jmp     .LBB7390_16
.LBB7390_16:
 movq    2800(%rsp), %rcx
 callq   pest::iterators::pair::Pair<R>::as_rule
 movb    %al, %cl
 movb    %cl, 2711(%rsp)
 jmp     .LBB7390_17
.LBB7390_17:
 movb    2711(%rsp), %al
 movb    %al, 3294(%rsp)
 leaq    3256(%rsp), %rax
 movq    %rax, 3224(%rsp)
 leaq    3264(%rsp), %rax
 movq    %rax, 3232(%rsp)
 leaq    3294(%rsp), %rax
 movq    %rax, 3240(%rsp)
 leaq    2840(%rsp), %rax
 movq    %rax, 3248(%rsp)
 movq    3224(%rsp), %rcx
 movq    %rcx, 118056(%rsp)
 movq    3232(%rsp), %rax
 movq    %rax, 2664(%rsp)
 movq    %rax, 118064(%rsp)
 movq    3240(%rsp), %rax
 movq    %rax, 2672(%rsp)
 movq    %rax, 118072(%rsp)
 movq    3248(%rsp), %rax
 movq    %rax, 2680(%rsp)
 movq    %rax, 118080(%rsp)
 leaq    _ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h8c09b1804c8ec62cE(%rip), %rdx
 callq   core::fmt::ArgumentV1::new
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2688(%rsp)
 movq    %rcx, 2696(%rsp)
 jmp     .LBB7390_20
.LBB7390_18:
 leaq    3264(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::string::String>
 jmp     .LBB7390_5
.LBB7390_19:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_18
.LBB7390_20:
 movq    2664(%rsp), %rcx
 movq    2688(%rsp), %rax
 movq    2696(%rsp), %rdx
 movq    %rdx, 2632(%rsp)
 movq    %rax, 2640(%rsp)
 leaq    _ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h20aa3722fd223d6cE(%rip), %rdx
 callq   core::fmt::ArgumentV1::new
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2648(%rsp)
 movq    %rcx, 2656(%rsp)
 jmp     .LBB7390_21
.LBB7390_21:
 movq    2672(%rsp), %rcx
 movq    2648(%rsp), %rax
 movq    2656(%rsp), %rdx
 movq    %rdx, 2600(%rsp)
 movq    %rax, 2608(%rsp)
 leaq    _ZN60_$LT$jodin_rs..parsing..Rule$u20$as$u20$core..fmt..Debug$GT$3fmt17h2e0ea3f7e451457fE(%rip), %rdx
 callq   core::fmt::ArgumentV1::new
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2616(%rsp)
 movq    %rcx, 2624(%rsp)
 jmp     .LBB7390_22
.LBB7390_22:
 movq    2680(%rsp), %rcx
 movq    2616(%rsp), %rax
 movq    2624(%rsp), %rdx
 movq    %rdx, 2568(%rsp)
 movq    %rax, 2576(%rsp)
 leaq    _ZN67_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17he8271684dacd979dE(%rip), %rdx
 callq   core::fmt::ArgumentV1::new
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2584(%rsp)
 movq    %rcx, 2592(%rsp)
 jmp     .LBB7390_23
.LBB7390_23:
 movq    2584(%rsp), %rcx
 movq    2592(%rsp), %rdx
 movq    2576(%rsp), %r8
 movq    2568(%rsp), %r9
 movq    2608(%rsp), %r10
 movq    2600(%rsp), %r11
 movq    2640(%rsp), %rsi
 movq    2632(%rsp), %rdi
 movq    %rdi, 3160(%rsp)
 movq    %rsi, 3168(%rsp)
 movq    %r11, 3176(%rsp)
 movq    %r10, 3184(%rsp)
 movq    %r9, 3192(%rsp)
 movq    %r8, 3200(%rsp)
 movq    %rdx, 3208(%rsp)
 movq    %rcx, 3216(%rsp)
 movq    %rsp, %rcx
 movq    $4, 32(%rcx)
 leaq    .L__unnamed_359(%rip), %rdx
 leaq    3112(%rsp), %rcx
 movl    $5, %r8d
 leaq    3160(%rsp), %r9
 callq   core::fmt::Arguments::new_v1
 jmp     .LBB7390_24
.LBB7390_24:
 leaq    3112(%rsp), %rcx
 callq   std::io::stdio::_print
 jmp     .LBB7390_25
.LBB7390_25:
 leaq    3264(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::string::String>
 jmp     .LBB7390_26
.LBB7390_26:
 jmp     .LBB7390_27
.LBB7390_27:
 movq    2760(%rsp), %rdx
 movq    2792(%rsp), %rcx
 addq    $24, %rcx
 callq   alloc::vec::Vec<T,A>::push
 jmp     .LBB7390_48
.LBB7390_28:
 movq    2712(%rsp), %rcx
 callq   core::num::<impl isize>::abs
 movq    %rax, %rcx
 movq    %rcx, 2560(%rsp)
 jmp     .LBB7390_30
.LBB7390_29:
 leaq    str.6(%rip), %rcx
 leaq    .L__unnamed_360(%rip), %r8
 movl    $33, %edx
 callq   core::panicking::panic
 jmp     .LBB7390_9
.LBB7390_30:
 movq    2560(%rsp), %rcx
 movq    %rcx, 118096(%rsp)
 callq   jodin_rs::utility::Bytes::new
 movq    %rax, %rcx
 movq    %rcx, 2552(%rsp)
 jmp     .LBB7390_31
.LBB7390_31:
 movq    2800(%rsp), %rcx
 movq    2552(%rsp), %rdx
 movq    %rdx, 2872(%rsp)
 movq    %rcx, 3048(%rsp)
 leaq    3056(%rsp), %rcx
 leaq    2872(%rsp), %rdx
 callq   <jodin_rs::utility::Bytes as jodin_rs::utility::HumanReadable>::human_readable
 jmp     .LBB7390_32
.LBB7390_32:
 leaq    3080(%rsp), %rcx
 leaq    2856(%rsp), %rdx
 callq   <jodin_rs::utility::Bytes as jodin_rs::utility::HumanReadable>::human_readable
 jmp     .LBB7390_33
.LBB7390_33:
 movq    2800(%rsp), %rcx
 callq   pest::iterators::pair::Pair<R>::as_rule
 movb    %al, %cl
 movb    %cl, 2551(%rsp)
 jmp     .LBB7390_36
.LBB7390_34:
 leaq    3056(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::string::String>
 jmp     .LBB7390_5
.LBB7390_35:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_34
.LBB7390_36:
 movb    2551(%rsp), %al
 movb    %al, 3111(%rsp)
 leaq    3048(%rsp), %rax
 movq    %rax, 3008(%rsp)
 leaq    3056(%rsp), %rax
 movq    %rax, 3016(%rsp)
 leaq    3080(%rsp), %rax
 movq    %rax, 3024(%rsp)
 leaq    3111(%rsp), %rax
 movq    %rax, 3032(%rsp)
 leaq    2840(%rsp), %rax
 movq    %rax, 3040(%rsp)
 movq    3008(%rsp), %rcx
 movq    %rcx, 118104(%rsp)
 movq    3016(%rsp), %rax
 movq    %rax, 2496(%rsp)
 movq    %rax, 118112(%rsp)
 movq    3024(%rsp), %rax
 movq    %rax, 2504(%rsp)
 movq    %rax, 118120(%rsp)
 movq    3032(%rsp), %rax
 movq    %rax, 2512(%rsp)
 movq    %rax, 118128(%rsp)
 movq    3040(%rsp), %rax
 movq    %rax, 2520(%rsp)
 movq    %rax, 118136(%rsp)
 leaq    _ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Pointer$GT$3fmt17h8c09b1804c8ec62cE(%rip), %rdx
 callq   core::fmt::ArgumentV1::new
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2528(%rsp)
 movq    %rcx, 2536(%rsp)
 jmp     .LBB7390_39
.LBB7390_37:
 leaq    3080(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::string::String>
 jmp     .LBB7390_34
.LBB7390_38:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_37
.LBB7390_39:
 movq    2496(%rsp), %rcx
 movq    2528(%rsp), %rax
 movq    2536(%rsp), %rdx
 movq    %rdx, 2464(%rsp)
 movq    %rax, 2472(%rsp)
 leaq    _ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h20aa3722fd223d6cE(%rip), %rdx
 callq   core::fmt::ArgumentV1::new
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2480(%rsp)
 movq    %rcx, 2488(%rsp)
 jmp     .LBB7390_40
.LBB7390_40:
 movq    2504(%rsp), %rcx
 movq    2480(%rsp), %rax
 movq    2488(%rsp), %rdx
 movq    %rdx, 2432(%rsp)
 movq    %rax, 2440(%rsp)
 leaq    _ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h20aa3722fd223d6cE(%rip), %rdx
 callq   core::fmt::ArgumentV1::new
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2448(%rsp)
 movq    %rcx, 2456(%rsp)
 jmp     .LBB7390_41
.LBB7390_41:
 movq    2512(%rsp), %rcx
 movq    2448(%rsp), %rax
 movq    2456(%rsp), %rdx
 movq    %rdx, 2400(%rsp)
 movq    %rax, 2408(%rsp)
 leaq    _ZN60_$LT$jodin_rs..parsing..Rule$u20$as$u20$core..fmt..Debug$GT$3fmt17h2e0ea3f7e451457fE(%rip), %rdx
 callq   core::fmt::ArgumentV1::new
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2416(%rsp)
 movq    %rcx, 2424(%rsp)
 jmp     .LBB7390_42
.LBB7390_42:
 movq    2520(%rsp), %rcx
 movq    2416(%rsp), %rax
 movq    2424(%rsp), %rdx
 movq    %rdx, 2368(%rsp)
 movq    %rax, 2376(%rsp)
 leaq    _ZN67_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17he8271684dacd979dE(%rip), %rdx
 callq   core::fmt::ArgumentV1::new
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2384(%rsp)
 movq    %rcx, 2392(%rsp)
 jmp     .LBB7390_43
.LBB7390_43:
 movq    2384(%rsp), %rcx
 movq    2392(%rsp), %rdx
 movq    2376(%rsp), %r8
 movq    2368(%rsp), %r9
 movq    2408(%rsp), %r10
 movq    2400(%rsp), %r11
 movq    2440(%rsp), %rsi
 movq    2432(%rsp), %rdi
 movq    2472(%rsp), %rbx
 movq    2464(%rsp), %r14
 movq    %r14, 2928(%rsp)
 movq    %rbx, 2936(%rsp)
 movq    %rdi, 2944(%rsp)
 movq    %rsi, 2952(%rsp)
 movq    %r11, 2960(%rsp)
 movq    %r10, 2968(%rsp)
 movq    %r9, 2976(%rsp)
 movq    %r8, 2984(%rsp)
 movq    %rdx, 2992(%rsp)
 movq    %rcx, 3000(%rsp)
 movq    %rsp, %rcx
 movq    $5, 32(%rcx)
 leaq    .L__unnamed_361(%rip), %rdx
 leaq    2880(%rsp), %rcx
 movl    $6, %r8d
 leaq    2928(%rsp), %r9
 callq   core::fmt::Arguments::new_v1
 jmp     .LBB7390_44
.LBB7390_44:
 leaq    2880(%rsp), %rcx
 callq   std::io::stdio::_print
 jmp     .LBB7390_45
.LBB7390_45:
 leaq    3080(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::string::String>
 jmp     .LBB7390_46
.LBB7390_46:
 leaq    3056(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::string::String>
 jmp     .LBB7390_47
.LBB7390_47:
 jmp     .LBB7390_27
.LBB7390_48:
 movq    2800(%rsp), %rcx
 callq   pest::iterators::pair::Pair<R>::as_rule
 movb    %al, %cl
 movb    %cl, 2367(%rsp)
 jmp     .LBB7390_49
.LBB7390_49:
 movq    2800(%rsp), %rdx
 movb    2367(%rsp), %cl
 movb    %cl, 3295(%rsp)
 leaq    3344(%rsp), %rcx
 callq   <pest::iterators::pair::Pair<R> as core::clone::Clone>::clone
 jmp     .LBB7390_50
.LBB7390_50:
 leaq    3304(%rsp), %rcx
 leaq    3344(%rsp), %rdx
 callq   pest::iterators::pair::Pair<R>::into_inner
 jmp     .LBB7390_51
.LBB7390_51:
 leaq    3304(%rsp), %rcx
 callq   jodin_rs::utility::IntoBox::boxed
 movq    %rax, %rcx
 movq    %rcx, 2352(%rsp)
 jmp     .LBB7390_52
.LBB7390_52:
 movq    2352(%rsp), %rax
 movq    %rax, 3296(%rsp)
 movb    $1, 117911(%rsp)
 movb    $1, 117912(%rsp)
 movzbl  3295(%rsp), %eax
 addq    $-39, %rax
 movq    %rax, 2344(%rsp)
 subq    $139, %rax
 ja      .LBB7390_53
 movq    2344(%rsp), %rax
 leaq    .LJTI7390_0(%rip), %rcx
 movslq  (%rcx, %rax, 4), %rax
 addq    %rcx, %rax
 jmpq    *%rax
.LBB7390_53:
 movq    2800(%rsp), %rcx
 movb    3295(%rsp), %al
 movb    %al, 2327(%rsp)
 movb    %al, 118148(%rsp)
 callq   pest::iterators::pair::Pair<R>::as_str
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2328(%rsp)
 movq    %rcx, 2336(%rsp)
 jmp     .LBB7390_84
.LBB7390_54:
 movb    $1, 9017(%rsp)
 movb    $2, 9016(%rsp)
 movups  9016(%rsp), %xmm0
 movups  9032(%rsp), %xmm1
 movups  %xmm1, 8824(%rsp)
 movups  %xmm0, 8808(%rsp)
 movb    $1, 8800(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    8800(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_1680
.LBB7390_55:
 movb    $0, 9265(%rsp)
 movb    $2, 9264(%rsp)
 movups  9264(%rsp), %xmm0
 movups  9280(%rsp), %xmm1
 movups  %xmm1, 9072(%rsp)
 movups  %xmm0, 9056(%rsp)
 movb    $1, 9048(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    9048(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_1679
.LBB7390_56:
 movq    2800(%rsp), %rcx
 movb    $0, 117914(%rsp)
 movups  (%rcx), %xmm0
 movups  16(%rcx), %xmm1
 movaps  %xmm1, 7120(%rsp)
 movaps  %xmm0, 7104(%rsp)
 leaq    7064(%rsp), %rcx
 leaq    7104(%rsp), %rdx
 callq   pest::iterators::pair::Pair<R>::into_inner
 jmp     .LBB7390_1672
.LBB7390_57:
 movq    2800(%rsp), %rcx
 callq   pest::iterators::pair::Pair<R>::as_str
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2304(%rsp)
 movq    %rcx, 2312(%rsp)
 jmp     .LBB7390_1669
.LBB7390_58:
 movq    2800(%rsp), %rcx
 callq   pest::iterators::pair::Pair<R>::as_str
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2288(%rsp)
 movq    %rcx, 2296(%rsp)
 jmp     .LBB7390_1642
.LBB7390_59:
 xorl    %ecx, %ecx
 movl    %ecx, %r8d
 leaq    113264(%rsp), %rcx
 leaq    3296(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_1628
.LBB7390_60:
 movq    2792(%rsp), %rdx
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %r8
 leaq    3680(%rsp), %rcx
 callq   jodin_rs::ast::JodinNodeGenerator::top_level_declarations
 jmp     .LBB7390_1620
.LBB7390_61:
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %rax
 movq    %rax, 29512(%rsp)
 cmpq    $2, 2848(%rsp)
 jae     .LBB7390_1437
 jmp     .LBB7390_1436
.LBB7390_62:
 leaq    114432(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1405
.LBB7390_63:
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %rcx
 movq    %rcx, 40608(%rsp)
 leaq    40648(%rsp), %rcx
 leaq    40608(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1360
.LBB7390_64:
 movq    2792(%rsp), %rdx
 movb    $0, 117913(%rsp)
 movq    2840(%rsp), %r8
 movq    2848(%rsp), %r9
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %r10
 movq    %rsp, %rcx
 movq    %r10, 32(%rcx)
 leaq    9560(%rsp), %rcx
 callq   jodin_rs::ast::JodinNodeGenerator::declaration
 jmp     .LBB7390_1352
.LBB7390_65:
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %rcx
 movq    32(%rcx), %rdx
 movq    %rdx, 102288(%rsp)
 movups  (%rcx), %xmm0
 movups  16(%rcx), %xmm1
 movaps  %xmm1, 102272(%rsp)
 movaps  %xmm0, 102256(%rsp)
 leaq    102208(%rsp), %rcx
 leaq    102256(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::new
 jmp     .LBB7390_1254
.LBB7390_66:
 leaq    116048(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1223
.LBB7390_67:
 cmpq    $2, 2848(%rsp)
 je      .LBB7390_1194
 jmp     .LBB7390_1193
.LBB7390_68:
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movb    $1, 117979(%rsp)
 movq    3296(%rsp), %rcx
 movq    %rcx, 47184(%rsp)
 leaq    47232(%rsp), %rcx
 leaq    47184(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1051
.LBB7390_69:
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %rax
 movq    %rax, 58312(%rsp)
 cmpq    $3, 2848(%rsp)
 je      .LBB7390_870
 jmp     .LBB7390_869
.LBB7390_70:
 cmpq    $2, 2848(%rsp)
 je      .LBB7390_832
 jmp     .LBB7390_831
.LBB7390_71:
 movq    2792(%rsp), %rdx
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %r8
 leaq    12216(%rsp), %rcx
 callq   jodin_rs::ast::JodinNodeGenerator::expression
 jmp     .LBB7390_823
.LBB7390_72:
 movq    2792(%rsp), %rdx
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %r8
 leaq    13288(%rsp), %rcx
 callq   jodin_rs::ast::JodinNodeGenerator::binop_expressions
 jmp     .LBB7390_815
.LBB7390_73:
 movq    2792(%rsp), %rdx
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %r8
 leaq    14360(%rsp), %rcx
 callq   jodin_rs::ast::JodinNodeGenerator::uni_op
 jmp     .LBB7390_807
.LBB7390_74:
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %rcx
 movq    32(%rcx), %rdx
 movq    %rdx, 15248(%rsp)
 movups  (%rcx), %xmm0
 movups  16(%rcx), %xmm1
 movaps  %xmm1, 15232(%rsp)
 movaps  %xmm0, 15216(%rsp)
 leaq    15168(%rsp), %rcx
 leaq    15216(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::new
 jmp     .LBB7390_758
.LBB7390_75:
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %rcx
 movq    %rcx, 110344(%rsp)
 leaq    110952(%rsp), %rcx
 leaq    110344(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_697
.LBB7390_76:
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %rcx
 movq    32(%rcx), %rdx
 movq    %rdx, 72768(%rsp)
 movups  (%rcx), %xmm0
 movups  16(%rcx), %xmm1
 movaps  %xmm1, 72752(%rsp)
 movaps  %xmm0, 72736(%rsp)
 leaq    72688(%rsp), %rcx
 leaq    72736(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::new
 jmp     .LBB7390_454
.LBB7390_77:
 movq    2784(%rsp), %rdx
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %rcx
 movq    %rcx, 92032(%rsp)
 xorl    %ecx, %ecx
 movl    %ecx, %r8d
 leaq    92040(%rsp), %rcx
 callq   alloc::vec::Vec<T,A>::remove
 jmp     .LBB7390_284
.LBB7390_78:
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %rcx
 movq    32(%rcx), %rdx
 movq    %rdx, 20304(%rsp)
 movups  (%rcx), %xmm0
 movups  16(%rcx), %xmm1
 movaps  %xmm1, 20288(%rsp)
 movaps  %xmm0, 20272(%rsp)
 leaq    20224(%rsp), %rcx
 leaq    20272(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::new
 jmp     .LBB7390_182
.LBB7390_79:
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %rcx
 movq    %rcx, 26720(%rsp)
 leaq    26760(%rsp), %rcx
 leaq    26720(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_137
.LBB7390_80:
 movq    2792(%rsp), %rdx
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %r8
 leaq    4752(%rsp), %rcx
 callq   jodin_rs::ast::JodinNodeGenerator::using_statement
 jmp     .LBB7390_129
.LBB7390_81:
 movq    2792(%rsp), %rdx
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %r8
 leaq    5824(%rsp), %rcx
 callq   jodin_rs::ast::JodinNodeGenerator::in_namespace
 jmp     .LBB7390_121
.LBB7390_82:
 movq    2792(%rsp), %rdx
 movq    2800(%rsp), %rcx
 movb    $0, 117914(%rsp)
 movups  (%rcx), %xmm0
 movups  16(%rcx), %xmm1
 movaps  %xmm1, 11008(%rsp)
 movaps  %xmm0, 10992(%rsp)
 leaq    10728(%rsp), %rcx
 leaq    10992(%rsp), %r8
 callq   jodin_rs::ast::JodinNodeGenerator::new_intermediate_type
 jmp     .LBB7390_109
.LBB7390_83:
 xorl    %ecx, %ecx
 movl    %ecx, %r8d
 leaq    108880(%rsp), %rcx
 leaq    3296(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_90
.LBB7390_84:
 movq    2328(%rsp), %r8
 movq    2336(%rsp), %rdx
 leaq    117880(%rsp), %rcx
 callq   <str as alloc::string::ToString>::to_string
 jmp     .LBB7390_87
.LBB7390_85:
 testb   $1, 117912(%rsp)
 jne     .LBB7390_1682
 jmp     .LBB7390_1681
.LBB7390_86:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_85
.LBB7390_87:
 movb    2327(%rsp), %cl
 movb    %cl, 117665(%rsp)
 movq    117896(%rsp), %rcx
 movq    %rcx, 117688(%rsp)
 movups  117880(%rsp), %xmm0
 movups  %xmm0, 117672(%rsp)
 movb    $27, 117664(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    117664(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_88
.LBB7390_88:
 jmp     .LBB7390_89
.LBB7390_89:
 movq    2808(%rsp), %rax
 movq    3376(%rsp), %rcx
 movq    %rcx, 8(%rax)
 movq    3384(%rsp), %rcx
 movq    %rcx, 16(%rax)
 movq    3392(%rsp), %rcx
 movq    %rcx, 24(%rax)
 movq    3400(%rsp), %rcx
 movq    %rcx, 32(%rax)
 movq    $0, (%rax)
 testb   $1, 117912(%rsp)
 jne     .LBB7390_1685
 jmp     .LBB7390_1684
.LBB7390_90:
 leaq    .L__unnamed_362(%rip), %r8
 leaq    108848(%rsp), %rcx
 leaq    108880(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_91
.LBB7390_91:
 movb    $1, 117930(%rsp)
 movups  108848(%rsp), %xmm0
 movups  108864(%rsp), %xmm1
 movaps  %xmm1, 109488(%rsp)
 movaps  %xmm0, 109472(%rsp)
 leaq    109512(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_92
.LBB7390_92:
 movq    2792(%rsp), %rdx
 movb    $0, 117930(%rsp)
 leaq    109208(%rsp), %rcx
 leaq    109472(%rsp), %r8
 leaq    109512(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_95
.LBB7390_93:
 testb   $1, 117930(%rsp)
 jne     .LBB7390_96
 jmp     .LBB7390_85
.LBB7390_94:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_93
.LBB7390_95:
 movb    $0, 117930(%rsp)
 leaq    108944(%rsp), %rcx
 leaq    109208(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_97
.LBB7390_96:
 leaq    109472(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_85
.LBB7390_97:
 movq    108944(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_99
 jmp     .LBB7390_1699
.LBB7390_1699:
 jmp     .LBB7390_100
 ud2
.LBB7390_99:
 movups  108952(%rsp), %xmm0
 movups  108968(%rsp), %xmm1
 movaps  %xmm1, 110064(%rsp)
 movaps  %xmm0, 110048(%rsp)
 movb    $1, 117929(%rsp)
 movaps  110048(%rsp), %xmm0
 movaps  110064(%rsp), %xmm1
 movaps  %xmm1, 108928(%rsp)
 movaps  %xmm0, 108912(%rsp)
 movb    $0, 117929(%rsp)
 movaps  108912(%rsp), %xmm0
 movaps  108928(%rsp), %xmm1
 movaps  %xmm1, 110320(%rsp)
 movaps  %xmm0, 110304(%rsp)
 movaps  110304(%rsp), %xmm0
 movaps  110320(%rsp), %xmm1
 movups  %xmm1, 110112(%rsp)
 movups  %xmm0, 110096(%rsp)
 movb    $5, 110088(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    110088(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_105
.LBB7390_100:
 leaq    108952(%rsp), %rdx
 leaq    109536(%rsp), %rcx
 movq    %rcx, 2264(%rsp)
 movl    $256, %r8d
 movq    %r8, 2272(%rsp)
 callq   memcpy
 movq    2264(%rsp), %rdx
 movq    2272(%rsp), %r8
 leaq    109792(%rsp), %rcx
 movq    %rcx, 2280(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    2280(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_101
.LBB7390_101:
 movb    $0, 117929(%rsp)
 jmp     .LBB7390_104
.LBB7390_102:
 jmp     .LBB7390_85
.LBB7390_103:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_102
.LBB7390_104:
 testb   $1, 117912(%rsp)
 jne     .LBB7390_1652
 jmp     .LBB7390_1651
.LBB7390_105:
 movb    $0, 117929(%rsp)
 jmp     .LBB7390_89
.LBB7390_106:
 testb   $1, 117929(%rsp)
 jne     .LBB7390_108
 jmp     .LBB7390_85
.LBB7390_107:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_106
.LBB7390_108:
 leaq    108912(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_85
.LBB7390_109:
 leaq    10464(%rsp), %rcx
 leaq    10728(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_110
.LBB7390_110:
 movq    10464(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_112
 jmp     .LBB7390_1700
.LBB7390_1700:
 jmp     .LBB7390_113
 ud2
.LBB7390_112:
 movups  10552(%rsp), %xmm0
 movaps  %xmm0, 11616(%rsp)
 movups  10536(%rsp), %xmm0
 movaps  %xmm0, 11600(%rsp)
 movups  10472(%rsp), %xmm0
 movups  10488(%rsp), %xmm1
 movups  10504(%rsp), %xmm2
 movups  10520(%rsp), %xmm3
 movaps  %xmm3, 11584(%rsp)
 movaps  %xmm2, 11568(%rsp)
 movaps  %xmm1, 11552(%rsp)
 movaps  %xmm0, 11536(%rsp)
 movb    $1, 118006(%rsp)
 movaps  11616(%rsp), %xmm0
 movaps  %xmm0, 10448(%rsp)
 movaps  11600(%rsp), %xmm0
 movaps  %xmm0, 10432(%rsp)
 movaps  11536(%rsp), %xmm0
 movaps  11552(%rsp), %xmm1
 movaps  11568(%rsp), %xmm2
 movaps  11584(%rsp), %xmm3
 movaps  %xmm3, 10416(%rsp)
 movaps  %xmm2, 10400(%rsp)
 movaps  %xmm1, 10384(%rsp)
 movaps  %xmm0, 10368(%rsp)
 movb    $0, 118006(%rsp)
 movaps  10448(%rsp), %xmm0
 movaps  %xmm0, 11936(%rsp)
 movaps  10432(%rsp), %xmm0
 movaps  %xmm0, 11920(%rsp)
 movaps  10368(%rsp), %xmm0
 movaps  10384(%rsp), %xmm1
 movaps  10400(%rsp), %xmm2
 movaps  10416(%rsp), %xmm3
 movaps  %xmm3, 11904(%rsp)
 movaps  %xmm2, 11888(%rsp)
 movaps  %xmm1, 11872(%rsp)
 movaps  %xmm0, 11856(%rsp)
 movaps  11936(%rsp), %xmm0
 movups  %xmm0, 11728(%rsp)
 movaps  11920(%rsp), %xmm0
 movups  %xmm0, 11712(%rsp)
 movaps  11856(%rsp), %xmm0
 movaps  11872(%rsp), %xmm1
 movaps  11888(%rsp), %xmm2
 movaps  11904(%rsp), %xmm3
 movups  %xmm3, 11696(%rsp)
 movups  %xmm2, 11680(%rsp)
 movups  %xmm1, 11664(%rsp)
 movups  %xmm0, 11648(%rsp)
 movb    $0, 11640(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    11640(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_117
.LBB7390_113:
 leaq    10472(%rsp), %rdx
 leaq    11024(%rsp), %rcx
 movq    %rcx, 2240(%rsp)
 movl    $256, %r8d
 movq    %r8, 2248(%rsp)
 callq   memcpy
 movq    2240(%rsp), %rdx
 movq    2248(%rsp), %r8
 leaq    11280(%rsp), %rcx
 movq    %rcx, 2256(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    2256(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_114
.LBB7390_114:
 movb    $0, 118006(%rsp)
 jmp     .LBB7390_104
.LBB7390_115:
 jmp     .LBB7390_85
.LBB7390_116:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_115
.LBB7390_117:
 movb    $0, 118006(%rsp)
 jmp     .LBB7390_89
.LBB7390_118:
 testb   $1, 118006(%rsp)
 jne     .LBB7390_120
 jmp     .LBB7390_85
.LBB7390_119:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_118
.LBB7390_120:
 leaq    10368(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::intermediate_type::IntermediateType>
 jmp     .LBB7390_85
.LBB7390_121:
 leaq    5560(%rsp), %rcx
 leaq    5824(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_122
.LBB7390_122:
 movq    5560(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_124
 jmp     .LBB7390_1701
.LBB7390_1701:
 jmp     .LBB7390_125
 ud2
.LBB7390_124:
 movq    5568(%rsp), %rax
 movq    %rax, 6600(%rsp)
 movq    5576(%rsp), %rax
 movq    %rax, 6608(%rsp)
 movq    5584(%rsp), %rax
 movq    %rax, 6616(%rsp)
 movq    5592(%rsp), %rax
 movq    %rax, 6624(%rsp)
 movq    6600(%rsp), %rax
 movq    %rax, 3376(%rsp)
 movq    6608(%rsp), %rax
 movq    %rax, 3384(%rsp)
 movq    6616(%rsp), %rax
 movq    %rax, 3392(%rsp)
 movq    6624(%rsp), %rax
 movq    %rax, 3400(%rsp)
 jmp     .LBB7390_89
.LBB7390_125:
 leaq    5568(%rsp), %rdx
 leaq    6088(%rsp), %rcx
 movq    %rcx, 2216(%rsp)
 movl    $256, %r8d
 movq    %r8, 2224(%rsp)
 callq   memcpy
 movq    2216(%rsp), %rdx
 movq    2224(%rsp), %r8
 leaq    6344(%rsp), %rcx
 movq    %rcx, 2232(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    2232(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_126
.LBB7390_126:
 jmp     .LBB7390_104
.LBB7390_127:
 jmp     .LBB7390_85
.LBB7390_128:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_127
.LBB7390_129:
 leaq    4488(%rsp), %rcx
 leaq    4752(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_130
.LBB7390_130:
 movq    4488(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_132
 jmp     .LBB7390_1702
.LBB7390_1702:
 jmp     .LBB7390_133
 ud2
.LBB7390_132:
 movq    4496(%rsp), %rax
 movq    %rax, 5528(%rsp)
 movq    4504(%rsp), %rax
 movq    %rax, 5536(%rsp)
 movq    4512(%rsp), %rax
 movq    %rax, 5544(%rsp)
 movq    4520(%rsp), %rax
 movq    %rax, 5552(%rsp)
 movq    5528(%rsp), %rax
 movq    %rax, 3376(%rsp)
 movq    5536(%rsp), %rax
 movq    %rax, 3384(%rsp)
 movq    5544(%rsp), %rax
 movq    %rax, 3392(%rsp)
 movq    5552(%rsp), %rax
 movq    %rax, 3400(%rsp)
 jmp     .LBB7390_89
.LBB7390_133:
 leaq    4496(%rsp), %rdx
 leaq    5016(%rsp), %rcx
 movq    %rcx, 2192(%rsp)
 movl    $256, %r8d
 movq    %r8, 2200(%rsp)
 callq   memcpy
 movq    2192(%rsp), %rdx
 movq    2200(%rsp), %r8
 leaq    5272(%rsp), %rcx
 movq    %rcx, 2208(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    2208(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_134
.LBB7390_134:
 jmp     .LBB7390_104
.LBB7390_135:
 jmp     .LBB7390_85
.LBB7390_136:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_135
.LBB7390_137:
 leaq    .L__unnamed_363(%rip), %r8
 leaq    26728(%rsp), %rcx
 leaq    26760(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_140
.LBB7390_138:
 leaq    26720(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_85
.LBB7390_139:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_138
.LBB7390_140:
 movb    $1, 117996(%rsp)
 leaq    26824(%rsp), %rcx
 leaq    26720(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_141
.LBB7390_141:
 leaq    .L__unnamed_364(%rip), %r8
 leaq    26792(%rsp), %rcx
 leaq    26824(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_144
.LBB7390_142:
 leaq    26728(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_138
.LBB7390_143:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_142
.LBB7390_144:
 movb    $1, 117997(%rsp)
 movups  26792(%rsp), %xmm0
 movups  26808(%rsp), %xmm1
 movaps  %xmm1, 27648(%rsp)
 movaps  %xmm0, 27632(%rsp)
 leaq    27672(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_145
.LBB7390_145:
 movq    2792(%rsp), %rdx
 movb    $0, 117997(%rsp)
 leaq    27368(%rsp), %rcx
 leaq    27632(%rsp), %r8
 leaq    27672(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_148
.LBB7390_146:
 testb   $1, 117997(%rsp)
 jne     .LBB7390_149
 jmp     .LBB7390_142
.LBB7390_147:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_146
.LBB7390_148:
 movb    $0, 117997(%rsp)
 leaq    27104(%rsp), %rcx
 leaq    27368(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_150
.LBB7390_149:
 leaq    27632(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_142
.LBB7390_150:
 movq    27104(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_152
 jmp     .LBB7390_1703
.LBB7390_1703:
 jmp     .LBB7390_153
 ud2
.LBB7390_152:
 movq    2792(%rsp), %rdx
 movups  27112(%rsp), %xmm0
 movups  27128(%rsp), %xmm1
 movaps  %xmm1, 28224(%rsp)
 movaps  %xmm0, 28208(%rsp)
 movaps  28208(%rsp), %xmm0
 movaps  28224(%rsp), %xmm1
 movaps  %xmm1, 27088(%rsp)
 movaps  %xmm0, 27072(%rsp)
 movb    $0, 117996(%rsp)
 movups  26728(%rsp), %xmm0
 movups  26744(%rsp), %xmm1
 movaps  %xmm1, 28880(%rsp)
 movaps  %xmm0, 28864(%rsp)
 leaq    28600(%rsp), %rcx
 leaq    28864(%rsp), %r8
 callq   jodin_rs::ast::JodinNodeGenerator::new_intermediate_type
 jmp     .LBB7390_163
.LBB7390_153:
 leaq    27112(%rsp), %rdx
 leaq    27696(%rsp), %rcx
 movq    %rcx, 2168(%rsp)
 movl    $256, %r8d
 movq    %r8, 2176(%rsp)
 callq   memcpy
 movq    2168(%rsp), %rdx
 movq    2176(%rsp), %r8
 leaq    27952(%rsp), %rcx
 movq    %rcx, 2184(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    2184(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_154
.LBB7390_154:
 leaq    26728(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_157
.LBB7390_155:
 testb   $1, 117996(%rsp)
 jne     .LBB7390_167
 jmp     .LBB7390_158
.LBB7390_156:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_155
.LBB7390_157:
 movb    $0, 117996(%rsp)
 leaq    26720(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_160
.LBB7390_158:
 leaq    26720(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_161
.LBB7390_159:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_158
.LBB7390_160:
 jmp     .LBB7390_104
.LBB7390_161:
 jmp     .LBB7390_85
.LBB7390_162:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_161
.LBB7390_163:
 leaq    28336(%rsp), %rcx
 leaq    28600(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_166
.LBB7390_164:
 leaq    27072(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_155
.LBB7390_165:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_164
.LBB7390_166:
 movq    28336(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_169
 jmp     .LBB7390_1704
.LBB7390_1704:
 jmp     .LBB7390_170
.LBB7390_167:
 leaq    26728(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_158
 ud2
.LBB7390_169:
 movups  28424(%rsp), %xmm0
 movaps  %xmm0, 29488(%rsp)
 movups  28408(%rsp), %xmm0
 movaps  %xmm0, 29472(%rsp)
 movups  28344(%rsp), %xmm0
 movups  28360(%rsp), %xmm1
 movups  28376(%rsp), %xmm2
 movups  28392(%rsp), %xmm3
 movaps  %xmm3, 29456(%rsp)
 movaps  %xmm2, 29440(%rsp)
 movaps  %xmm1, 29424(%rsp)
 movaps  %xmm0, 29408(%rsp)
 movaps  29488(%rsp), %xmm0
 movaps  %xmm0, 28320(%rsp)
 movaps  29472(%rsp), %xmm0
 movaps  %xmm0, 28304(%rsp)
 movaps  29408(%rsp), %xmm0
 movaps  29424(%rsp), %xmm1
 movaps  29440(%rsp), %xmm2
 movaps  29456(%rsp), %xmm3
 movaps  %xmm3, 28288(%rsp)
 movaps  %xmm2, 28272(%rsp)
 movaps  %xmm1, 28256(%rsp)
 movaps  %xmm0, 28240(%rsp)
 movaps  27072(%rsp), %xmm0
 movaps  27088(%rsp), %xmm1
 movups  %xmm1, 26880(%rsp)
 movups  %xmm0, 26864(%rsp)
 movaps  28320(%rsp), %xmm0
 movups  %xmm0, 26976(%rsp)
 movaps  28304(%rsp), %xmm0
 movups  %xmm0, 26960(%rsp)
 movaps  28240(%rsp), %xmm0
 movaps  28256(%rsp), %xmm1
 movaps  28272(%rsp), %xmm2
 movaps  28288(%rsp), %xmm3
 movups  %xmm3, 26944(%rsp)
 movups  %xmm2, 26928(%rsp)
 movups  %xmm1, 26912(%rsp)
 movups  %xmm0, 26896(%rsp)
 movb    $8, 26856(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    26856(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_180
.LBB7390_170:
 leaq    28344(%rsp), %rdx
 leaq    28896(%rsp), %rcx
 movq    %rcx, 2144(%rsp)
 movl    $256, %r8d
 movq    %r8, 2152(%rsp)
 callq   memcpy
 movq    2144(%rsp), %rdx
 movq    2152(%rsp), %r8
 leaq    29152(%rsp), %rcx
 movq    %rcx, 2160(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    2160(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_171
.LBB7390_171:
 leaq    27072(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_175
.LBB7390_172:
 leaq    27072(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_174
.LBB7390_173:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_172
.LBB7390_174:
 leaq    26720(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_178
.LBB7390_175:
 movb    $0, 117996(%rsp)
 leaq    26720(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_177
.LBB7390_176:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_174
.LBB7390_177:
 jmp     .LBB7390_160
.LBB7390_178:
 jmp     .LBB7390_161
.LBB7390_179:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_178
.LBB7390_180:
 movb    $0, 117996(%rsp)
 leaq    26720(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_181
.LBB7390_181:
 jmp     .LBB7390_89
.LBB7390_182:
 cmpq    $3, 2848(%rsp)
 jae     .LBB7390_184
.LBB7390_183:
 cmpq    $2, 2848(%rsp)
 jae     .LBB7390_205
 jmp     .LBB7390_204
.LBB7390_184:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 cmpq    $173, %rax
 jne     .LBB7390_183
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $23, %rax
 jne     .LBB7390_183
 movq    2840(%rsp), %rax
 movzbl  2(%rax), %eax
 cmpq    $99, %rax
 jne     .LBB7390_183
 movb    $-83, 20807(%rsp)
 movb    20807(%rsp), %r8b
 leaq    20536(%rsp), %rcx
 leaq    20224(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_188
.LBB7390_188:
 leaq    .L__unnamed_365(%rip), %r8
 leaq    20504(%rsp), %rcx
 leaq    20536(%rsp), %rdx
 callq   core::result::Result<T,E>::unwrap
 jmp     .LBB7390_191
.LBB7390_189:
 leaq    20224(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_85
.LBB7390_190:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_189
.LBB7390_191:
 leaq    20464(%rsp), %rcx
 leaq    20504(%rsp), %rdx
 callq   pest::iterators::pair::Pair<R>::into_inner
 jmp     .LBB7390_192
.LBB7390_192:
 leaq    20432(%rsp), %rcx
 leaq    20464(%rsp), %rdx
 callq   <pest::iterators::pairs::Pairs<R> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_193
.LBB7390_193:
 leaq    .L__unnamed_366(%rip), %r8
 leaq    20400(%rsp), %rcx
 leaq    20432(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_196
.LBB7390_194:
 leaq    20464(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_189
.LBB7390_195:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_194
.LBB7390_196:
 leaq    20400(%rsp), %rcx
 callq   pest::iterators::pair::Pair<R>::as_rule
 movb    %al, %cl
 movb    %cl, 2143(%rsp)
 jmp     .LBB7390_197
.LBB7390_197:
 movb    2143(%rsp), %cl
 movb    %cl, 20399(%rsp)
 movb    $99, 21111(%rsp)
 movb    21111(%rsp), %r8b
 leaq    20840(%rsp), %rcx
 leaq    20224(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_200
.LBB7390_198:
 leaq    20400(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_194
.LBB7390_199:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_198
.LBB7390_200:
 leaq    .L__unnamed_367(%rip), %r8
 leaq    20808(%rsp), %rcx
 leaq    20840(%rsp), %rdx
 callq   core::result::Result<T,E>::unwrap
 jmp     .LBB7390_201
.LBB7390_201:
 movb    20399(%rsp), %cl
 movb    %cl, 20352(%rsp)
 movups  20808(%rsp), %xmm0
 movups  20824(%rsp), %xmm1
 movups  %xmm1, 20376(%rsp)
 movups  %xmm0, 20360(%rsp)
 leaq    20400(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_202
.LBB7390_202:
 leaq    20464(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_203
.LBB7390_203:
 movb    20352(%rsp), %cl
 movb    %cl, 2142(%rsp)
 movb    %cl, 118149(%rsp)
 movb    $1, 117917(%rsp)
 movups  20360(%rsp), %xmm0
 movups  20376(%rsp), %xmm1
 movaps  %xmm1, 20336(%rsp)
 movaps  %xmm0, 20320(%rsp)
 leaq    21416(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_210
.LBB7390_204:
 leaq    .L__unnamed_181(%rip), %rcx
 leaq    .L__unnamed_368(%rip), %r8
 movl    $40, %edx
 callq   core::panicking::panic
 jmp     .LBB7390_9
.LBB7390_205:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 cmpq    $23, %rax
 jne     .LBB7390_204
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $99, %rax
 jne     .LBB7390_204
 movb    $99, 21415(%rsp)
 movb    21415(%rsp), %r8b
 leaq    21144(%rsp), %rcx
 leaq    20224(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_208
.LBB7390_208:
 leaq    .L__unnamed_369(%rip), %r8
 leaq    21112(%rsp), %rcx
 leaq    21144(%rsp), %rdx
 callq   core::result::Result<T,E>::unwrap
 jmp     .LBB7390_209
.LBB7390_209:
 movb    $-73, 20352(%rsp)
 movq    21112(%rsp), %rax
 movq    %rax, 20360(%rsp)
 movq    21120(%rsp), %rax
 movq    %rax, 20368(%rsp)
 movq    21128(%rsp), %rax
 movq    %rax, 20376(%rsp)
 movq    21136(%rsp), %rax
 movq    %rax, 20384(%rsp)
 jmp     .LBB7390_203
.LBB7390_210:
 movb    $1, 118002(%rsp)
 movb    $-94, 21447(%rsp)
 movb    21447(%rsp), %dl
 leaq    20224(%rsp), %rcx
 callq   jodin_rs::ast::IndexedPair::contains
 movb    %al, %cl
 movb    %cl, 2141(%rsp)
 jmp     .LBB7390_213
.LBB7390_211:
 testb   $1, 117917(%rsp)
 jne     .LBB7390_283
 jmp     .LBB7390_189
.LBB7390_212:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_211
.LBB7390_213:
 movb    2141(%rsp), %al
 testb   $1, %al
 jne     .LBB7390_217
 jmp     .LBB7390_216
.LBB7390_214:
 testb   $1, 118002(%rsp)
 jne     .LBB7390_282
 jmp     .LBB7390_211
.LBB7390_215:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_214
.LBB7390_216:
 jmp     .LBB7390_244
.LBB7390_217:
 movb    $-94, 21743(%rsp)
 movb    21743(%rsp), %r8b
 leaq    21472(%rsp), %rcx
 leaq    20224(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get_all
 jmp     .LBB7390_218
.LBB7390_218:
 leaq    .L__unnamed_370(%rip), %r8
 leaq    21448(%rsp), %rcx
 leaq    21472(%rsp), %rdx
 callq   core::result::Result<T,E>::unwrap
 jmp     .LBB7390_219
.LBB7390_219:
 movq    21464(%rsp), %rcx
 movq    %rcx, 21792(%rsp)
 movups  21448(%rsp), %xmm0
 movaps  %xmm0, 21776(%rsp)
 leaq    21744(%rsp), %rcx
 leaq    21776(%rsp), %rdx
 callq   <alloc::vec::Vec<T,A> as core::iter::traits::collect::IntoIterator>::into_iter
 jmp     .LBB7390_220
.LBB7390_220:
 movq    21744(%rsp), %rax
 movq    %rax, 21808(%rsp)
 movq    21752(%rsp), %rax
 movq    %rax, 21816(%rsp)
 movq    21760(%rsp), %rax
 movq    %rax, 21824(%rsp)
 movq    21768(%rsp), %rax
 movq    %rax, 21832(%rsp)
.LBB7390_221:
 leaq    21872(%rsp), %rcx
 leaq    21808(%rsp), %rdx
 callq   <alloc::vec::into_iter::IntoIter<T,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_222
.LBB7390_222:
 movq    21872(%rsp), %rax
 testq   %rax, %rax
 setne   %al
 movzbl  %al, %eax
 je      .LBB7390_226
 jmp     .LBB7390_1705
.LBB7390_1705:
 jmp     .LBB7390_227
.LBB7390_223:
 testb   $1, 118001(%rsp)
 jne     .LBB7390_243
 jmp     .LBB7390_242
.LBB7390_224:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_223
 ud2
.LBB7390_226:
 movb    $0, 118001(%rsp)
 leaq    21808(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_244
.LBB7390_227:
 movups  21872(%rsp), %xmm0
 movups  21888(%rsp), %xmm1
 movaps  %xmm1, 21920(%rsp)
 movaps  %xmm0, 21904(%rsp)
 movaps  21904(%rsp), %xmm0
 movaps  21920(%rsp), %xmm1
 movaps  %xmm1, 21952(%rsp)
 movaps  %xmm0, 21936(%rsp)
 movb    $1, 118001(%rsp)
 movaps  21936(%rsp), %xmm0
 movaps  21952(%rsp), %xmm1
 movaps  %xmm1, 21856(%rsp)
 movaps  %xmm0, 21840(%rsp)
 movb    $0, 118001(%rsp)
 movaps  21840(%rsp), %xmm0
 movaps  21856(%rsp), %xmm1
 movaps  %xmm1, 21984(%rsp)
 movaps  %xmm0, 21968(%rsp)
 movb    $1, 118000(%rsp)
 movaps  21968(%rsp), %xmm0
 movaps  21984(%rsp), %xmm1
 movaps  %xmm1, 22576(%rsp)
 movaps  %xmm0, 22560(%rsp)
 leaq    22600(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_228
.LBB7390_228:
 movq    2792(%rsp), %rdx
 movb    $0, 118000(%rsp)
 leaq    22296(%rsp), %rcx
 leaq    22560(%rsp), %r8
 leaq    22600(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_231
.LBB7390_229:
 testb   $1, 118000(%rsp)
 jne     .LBB7390_232
 jmp     .LBB7390_223
.LBB7390_230:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_229
.LBB7390_231:
 movb    $0, 118000(%rsp)
 leaq    22032(%rsp), %rcx
 leaq    22296(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_233
.LBB7390_232:
 leaq    22560(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_223
.LBB7390_233:
 movq    22032(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_235
 jmp     .LBB7390_1706
.LBB7390_1706:
 jmp     .LBB7390_236
 ud2
.LBB7390_235:
 movups  22040(%rsp), %xmm0
 movups  22056(%rsp), %xmm1
 movaps  %xmm1, 23152(%rsp)
 movaps  %xmm0, 23136(%rsp)
 movaps  23136(%rsp), %xmm0
 movaps  23152(%rsp), %xmm1
 movaps  %xmm1, 22016(%rsp)
 movaps  %xmm0, 22000(%rsp)
 leaq    21416(%rsp), %rcx
 leaq    22000(%rsp), %rdx
 callq   alloc::vec::Vec<T,A>::push
 jmp     .LBB7390_241
.LBB7390_236:
 leaq    22040(%rsp), %rdx
 leaq    22624(%rsp), %rcx
 movq    %rcx, 2112(%rsp)
 movl    $256, %r8d
 movq    %r8, 2120(%rsp)
 callq   memcpy
 movq    2112(%rsp), %rdx
 movq    2120(%rsp), %r8
 leaq    22880(%rsp), %rcx
 movq    %rcx, 2128(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    2128(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_237
.LBB7390_237:
 movb    $0, 118001(%rsp)
 leaq    21808(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_240
.LBB7390_238:
 jmp     .LBB7390_223
.LBB7390_239:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_238
.LBB7390_240:
 leaq    21416(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_277
.LBB7390_241:
 movb    $0, 118001(%rsp)
 jmp     .LBB7390_221
.LBB7390_242:
 leaq    21808(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_214
.LBB7390_243:
 leaq    21840(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_242
.LBB7390_244:
 movb    2142(%rsp), %dl
 leaq    23432(%rsp), %rcx
 callq   <jodin_rs::core::privacy::Visibility as core::convert::TryFrom<core::option::Option<jodin_rs::parsing::Rule>>>::try_from
 jmp     .LBB7390_245
.LBB7390_245:
 leaq    23168(%rsp), %rcx
 leaq    23432(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_246
.LBB7390_246:
 movzbl  23168(%rsp), %eax
 testb   $1, %al
 je      .LBB7390_248
 jmp     .LBB7390_1707
.LBB7390_1707:
 jmp     .LBB7390_249
 ud2
.LBB7390_248:
 movb    23169(%rsp), %cl
 movb    %cl, 2111(%rsp)
 movb    %cl, 118150(%rsp)
 movb    %cl, 118151(%rsp)
 movb    $0, 117917(%rsp)
 movb    $1, 117999(%rsp)
 movaps  20320(%rsp), %xmm0
 movaps  20336(%rsp), %xmm1
 movaps  %xmm1, 24784(%rsp)
 movaps  %xmm0, 24768(%rsp)
 leaq    24808(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_253
.LBB7390_249:
 leaq    23176(%rsp), %rdx
 leaq    23696(%rsp), %rcx
 movq    %rcx, 2080(%rsp)
 movl    $256, %r8d
 movq    %r8, 2088(%rsp)
 callq   memcpy
 movq    2080(%rsp), %rdx
 movq    2088(%rsp), %r8
 leaq    23952(%rsp), %rcx
 movq    %rcx, 2096(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    2096(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_250
.LBB7390_250:
 jmp     .LBB7390_240
.LBB7390_251:
 jmp     .LBB7390_214
.LBB7390_252:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_251
.LBB7390_253:
 movq    2792(%rsp), %rdx
 movb    $0, 117999(%rsp)
 leaq    24504(%rsp), %rcx
 leaq    24768(%rsp), %r8
 leaq    24808(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_256
.LBB7390_254:
 testb   $1, 117999(%rsp)
 jne     .LBB7390_257
 jmp     .LBB7390_214
.LBB7390_255:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_254
.LBB7390_256:
 movb    $0, 117999(%rsp)
 leaq    24240(%rsp), %rcx
 leaq    24504(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_258
.LBB7390_257:
 leaq    24768(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_214
.LBB7390_258:
 movq    24240(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_260
 jmp     .LBB7390_1708
.LBB7390_1708:
 jmp     .LBB7390_261
 ud2
.LBB7390_260:
 movb    2111(%rsp), %cl
 movups  24248(%rsp), %xmm0
 movups  24264(%rsp), %xmm1
 movaps  %xmm1, 25360(%rsp)
 movaps  %xmm0, 25344(%rsp)
 movb    $1, 117998(%rsp)
 movaps  25344(%rsp), %xmm0
 movaps  25360(%rsp), %xmm1
 movaps  %xmm1, 24224(%rsp)
 movaps  %xmm0, 24208(%rsp)
 callq   jodin_rs::core::privacy::VisibilityTag::new
 movb    %al, %cl
 movb    %cl, 2079(%rsp)
 jmp     .LBB7390_266
.LBB7390_261:
 leaq    24248(%rsp), %rdx
 leaq    24832(%rsp), %rcx
 movq    %rcx, 2048(%rsp)
 movl    $256, %r8d
 movq    %r8, 2056(%rsp)
 callq   memcpy
 movq    2048(%rsp), %rdx
 movq    2056(%rsp), %r8
 leaq    25088(%rsp), %rcx
 movq    %rcx, 2064(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    2064(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_262
.LBB7390_262:
 jmp     .LBB7390_265
.LBB7390_263:
 jmp     .LBB7390_214
.LBB7390_264:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_263
.LBB7390_265:
 movb    $0, 117998(%rsp)
 jmp     .LBB7390_240
.LBB7390_266:
 movb    2079(%rsp), %r8b
 leaq    25640(%rsp), %rcx
 leaq    24208(%rsp), %rdx
 callq   jodin_rs::ast::jodin_node::JodinNode::add_tag
 jmp     .LBB7390_269
.LBB7390_267:
 testb   $1, 117998(%rsp)
 jne     .LBB7390_281
 jmp     .LBB7390_214
.LBB7390_268:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_267
.LBB7390_269:
 leaq    25384(%rsp), %rcx
 leaq    25640(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_270
.LBB7390_270:
 movb    25384(%rsp), %al
 addb    $-23, %al
 setne   %al
 movzbl  %al, %eax
 je      .LBB7390_272
 jmp     .LBB7390_1709
.LBB7390_1709:
 jmp     .LBB7390_273
 ud2
.LBB7390_272:
 movb    $0, 117998(%rsp)
 movaps  24208(%rsp), %xmm0
 movaps  24224(%rsp), %xmm1
 movaps  %xmm1, 26672(%rsp)
 movaps  %xmm0, 26656(%rsp)
 movb    $0, 118002(%rsp)
 movq    21432(%rsp), %rcx
 movq    %rcx, 26704(%rsp)
 movups  21416(%rsp), %xmm0
 movaps  %xmm0, 26688(%rsp)
 movaps  26656(%rsp), %xmm0
 movaps  26672(%rsp), %xmm1
 movups  %xmm1, 26464(%rsp)
 movups  %xmm0, 26448(%rsp)
 movq    26704(%rsp), %rcx
 movq    %rcx, 26496(%rsp)
 movaps  26688(%rsp), %xmm0
 movups  %xmm0, 26480(%rsp)
 movb    $7, 26440(%rsp)
 leaq    26408(%rsp), %rcx
 leaq    26440(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_280
.LBB7390_273:
 leaq    25896(%rsp), %rcx
 movq    %rcx, 2024(%rsp)
 leaq    25384(%rsp), %rdx
 movl    $256, %r8d
 movq    %r8, 2032(%rsp)
 callq   memcpy
 movq    2024(%rsp), %rdx
 movq    2032(%rsp), %r8
 leaq    26152(%rsp), %rcx
 movq    %rcx, 2040(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    2040(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_274
.LBB7390_274:
 leaq    24208(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_265
.LBB7390_275:
 jmp     .LBB7390_267
.LBB7390_276:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_275
.LBB7390_277:
 movb    $0, 118002(%rsp)
 testb   $1, 117917(%rsp)
 jne     .LBB7390_279
.LBB7390_278:
 movb    $0, 117917(%rsp)
 leaq    20224(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_104
.LBB7390_279:
 leaq    20320(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_278
.LBB7390_280:
 movups  26408(%rsp), %xmm0
 movups  26424(%rsp), %xmm1
 movaps  %xmm1, 3392(%rsp)
 movaps  %xmm0, 3376(%rsp)
 movb    $0, 117998(%rsp)
 movb    $0, 118002(%rsp)
 movb    $0, 117917(%rsp)
 leaq    20224(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_89
.LBB7390_281:
 leaq    24208(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_214
.LBB7390_282:
 leaq    21416(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_211
.LBB7390_283:
 leaq    20320(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_189
.LBB7390_284:
 movb    $1, 117915(%rsp)
 cmpq    $1, 2848(%rsp)
 je      .LBB7390_288
 jmp     .LBB7390_287
.LBB7390_285:
 leaq    92032(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_85
.LBB7390_286:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_285
.LBB7390_287:
 cmpq    $2, 2848(%rsp)
 je      .LBB7390_329
 jmp     .LBB7390_289
.LBB7390_288:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 movq    %rax, 2016(%rsp)
 subq    $52, %rax
 je      .LBB7390_290
 jmp     .LBB7390_1716
.LBB7390_1716:
 movq    2016(%rsp), %rax
 subq    $53, %rax
 je      .LBB7390_291
 jmp     .LBB7390_1717
.LBB7390_1717:
 movq    2016(%rsp), %rax
 subq    $158, %rax
 je      .LBB7390_292
 jmp     .LBB7390_289
.LBB7390_289:
 leaq    2840(%rsp), %rax
 movq    %rax, 100968(%rsp)
 movq    100968(%rsp), %rcx
 movq    %rcx, 118152(%rsp)
 leaq    _ZN67_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17he8271684dacd979dE(%rip), %rdx
 callq   core::fmt::ArgumentV1::new
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 2000(%rsp)
 movq    %rcx, 2008(%rsp)
 jmp     .LBB7390_451
.LBB7390_290:
 movq    2808(%rsp), %rcx
 movb    $0, 117915(%rsp)
 movups  92040(%rsp), %xmm0
 movups  92056(%rsp), %xmm1
 movaps  %xmm1, 92592(%rsp)
 movaps  %xmm0, 92576(%rsp)
 movb    $2, 92361(%rsp)
 movaps  92576(%rsp), %xmm0
 movaps  92592(%rsp), %xmm1
 movups  %xmm1, 92384(%rsp)
 movups  %xmm0, 92368(%rsp)
 movb    $11, 92360(%rsp)
 leaq    92360(%rsp), %rdx
 callq   jodin_rs::ast::node_type::JodinNodeType::into_result
 jmp     .LBB7390_328
.LBB7390_291:
 movq    2808(%rsp), %rcx
 movb    $0, 117915(%rsp)
 movups  92040(%rsp), %xmm0
 movups  92056(%rsp), %xmm1
 movaps  %xmm1, 92336(%rsp)
 movaps  %xmm0, 92320(%rsp)
 movb    $1, 92105(%rsp)
 movaps  92320(%rsp), %xmm0
 movaps  92336(%rsp), %xmm1
 movups  %xmm1, 92128(%rsp)
 movups  %xmm0, 92112(%rsp)
 movb    $11, 92104(%rsp)
 leaq    92104(%rsp), %rdx
 callq   jodin_rs::ast::node_type::JodinNodeType::into_result
 jmp     .LBB7390_327
.LBB7390_292:
 leaq    96696(%rsp), %rcx
 leaq    92032(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_293
.LBB7390_293:
 leaq    .L__unnamed_371(%rip), %r8
 leaq    96664(%rsp), %rcx
 leaq    96696(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_296
.LBB7390_294:
 testb   $1, 117915(%rsp)
 jne     .LBB7390_453
 jmp     .LBB7390_285
.LBB7390_295:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_294
.LBB7390_296:
 movb    $1, 117947(%rsp)
 leaq    96728(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_297
.LBB7390_297:
 movq    2792(%rsp), %rdx
 movb    $0, 117947(%rsp)
 leaq    96400(%rsp), %rcx
 leaq    96664(%rsp), %r8
 leaq    96728(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_300
.LBB7390_298:
 testb   $1, 117947(%rsp)
 jne     .LBB7390_301
 jmp     .LBB7390_294
.LBB7390_299:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_298
.LBB7390_300:
 movb    $0, 117947(%rsp)
 leaq    96136(%rsp), %rcx
 leaq    96400(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_302
.LBB7390_301:
 leaq    96664(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_294
.LBB7390_302:
 movq    96136(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_304
 jmp     .LBB7390_1718
.LBB7390_1718:
 jmp     .LBB7390_305
 ud2
.LBB7390_304:
 movups  96144(%rsp), %xmm0
 movups  96160(%rsp), %xmm1
 movaps  %xmm1, 97280(%rsp)
 movaps  %xmm0, 97264(%rsp)
 movaps  97264(%rsp), %xmm0
 movaps  97280(%rsp), %xmm1
 movaps  %xmm1, 96112(%rsp)
 movaps  %xmm0, 96096(%rsp)
 leaq    95880(%rsp), %rcx
 leaq    96096(%rsp), %rdx
 callq   jodin_rs::ast::jodin_node::JodinNode::into_inner
 jmp     .LBB7390_310
.LBB7390_305:
 leaq    96144(%rsp), %rdx
 leaq    96752(%rsp), %rcx
 movq    %rcx, 1976(%rsp)
 movl    $256, %r8d
 movq    %r8, 1984(%rsp)
 callq   memcpy
 movq    1976(%rsp), %rdx
 movq    1984(%rsp), %r8
 leaq    97008(%rsp), %rcx
 movq    %rcx, 1992(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1992(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_306
.LBB7390_306:
 movb    $0, 117946(%rsp)
 jmp     .LBB7390_309
.LBB7390_307:
 jmp     .LBB7390_294
.LBB7390_308:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_307
.LBB7390_309:
 movb    $0, 117941(%rsp)
 testb   $1, 117915(%rsp)
 jne     .LBB7390_449
 jmp     .LBB7390_448
.LBB7390_310:
 movzbl  95880(%rsp), %eax
 cmpq    $20, %rax
 jne     .LBB7390_312
 movq    95888(%rsp), %rax
 movq    %rax, 97296(%rsp)
 movq    95896(%rsp), %rax
 movq    %rax, 97304(%rsp)
 movq    95904(%rsp), %rax
 movq    %rax, 97312(%rsp)
 movb    $1, 117946(%rsp)
 movq    97296(%rsp), %rax
 movq    %rax, 95856(%rsp)
 movq    97304(%rsp), %rax
 movq    %rax, 95864(%rsp)
 movq    97312(%rsp), %rax
 movq    %rax, 95872(%rsp)
 movzbl  95880(%rsp), %eax
 cmpq    $20, %rax
 je      .LBB7390_315
 jmp     .LBB7390_316
.LBB7390_312:
 leaq    .L__unnamed_372(%rip), %rcx
 leaq    .L__unnamed_373(%rip), %r8
 movl    $34, %edx
 callq   std::panicking::begin_panic
 jmp     .LBB7390_9
.LBB7390_313:
 leaq    95880(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_307
.LBB7390_314:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_313
.LBB7390_315:
 movb    $0, 117915(%rsp)
 movups  92040(%rsp), %xmm0
 movups  92056(%rsp), %xmm1
 movaps  %xmm1, 97552(%rsp)
 movaps  %xmm0, 97536(%rsp)
 leaq    97576(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_320
.LBB7390_316:
 leaq    95880(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_315
.LBB7390_317:
 jmp     .LBB7390_319
.LBB7390_318:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_317
.LBB7390_319:
 testb   $1, 117946(%rsp)
 jne     .LBB7390_325
 jmp     .LBB7390_294
.LBB7390_320:
 movb    $0, 117946(%rsp)
 movq    95872(%rsp), %rcx
 movq    %rcx, 97616(%rsp)
 movups  95856(%rsp), %xmm0
 movaps  %xmm0, 97600(%rsp)
 movaps  97536(%rsp), %xmm0
 movaps  97552(%rsp), %xmm1
 movups  %xmm1, 97344(%rsp)
 movups  %xmm0, 97328(%rsp)
 movq    97592(%rsp), %rcx
 movq    %rcx, 97376(%rsp)
 movups  97576(%rsp), %xmm0
 movups  %xmm0, 97360(%rsp)
 movq    97616(%rsp), %rcx
 movq    %rcx, 97400(%rsp)
 movaps  97600(%rsp), %xmm0
 movups  %xmm0, 97384(%rsp)
 movb    $15, 97320(%rsp)
 leaq    92072(%rsp), %rcx
 leaq    97320(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_323
.LBB7390_321:
 leaq    97536(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_319
.LBB7390_322:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_321
.LBB7390_323:
 movb    $1, 117941(%rsp)
 movb    $0, 117946(%rsp)
 jmp     .LBB7390_326
.LBB7390_324:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_319
.LBB7390_325:
 leaq    95856(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_294
.LBB7390_326:
 leaq    100976(%rsp), %rcx
 leaq    92032(%rsp), %rdx
 callq   core::iter::traits::iterator::Iterator::find
 jmp     .LBB7390_427
.LBB7390_327:
 jmp     .LBB7390_309
.LBB7390_328:
 jmp     .LBB7390_309
.LBB7390_329:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 movq    %rax, 1968(%rsp)
 subq    $96, %rax
 je      .LBB7390_330
 jmp     .LBB7390_1710
.LBB7390_1710:
 movq    1968(%rsp), %rax
 subq    $97, %rax
 je      .LBB7390_331
 jmp     .LBB7390_1711
.LBB7390_1711:
 movq    1968(%rsp), %rax
 subq    $159, %rax
 je      .LBB7390_332
 jmp     .LBB7390_289
.LBB7390_330:
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $99, %rax
 je      .LBB7390_408
 jmp     .LBB7390_289
.LBB7390_331:
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $99, %rax
 je      .LBB7390_388
 jmp     .LBB7390_289
.LBB7390_332:
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $158, %rax
 jne     .LBB7390_289
 leaq    93448(%rsp), %rcx
 leaq    92032(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_334
.LBB7390_334:
 leaq    .L__unnamed_374(%rip), %r8
 leaq    93416(%rsp), %rcx
 leaq    93448(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_335
.LBB7390_335:
 movb    $1, 117951(%rsp)
 leaq    93480(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_336
.LBB7390_336:
 movq    2792(%rsp), %rdx
 movb    $0, 117951(%rsp)
 leaq    93152(%rsp), %rcx
 leaq    93416(%rsp), %r8
 leaq    93480(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_339
.LBB7390_337:
 testb   $1, 117951(%rsp)
 jne     .LBB7390_340
 jmp     .LBB7390_294
.LBB7390_338:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_337
.LBB7390_339:
 movb    $0, 117951(%rsp)
 leaq    92888(%rsp), %rcx
 leaq    93152(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_341
.LBB7390_340:
 leaq    93416(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_294
.LBB7390_341:
 movq    92888(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_343
 jmp     .LBB7390_1712
.LBB7390_1712:
 jmp     .LBB7390_344
 ud2
.LBB7390_343:
 movups  92896(%rsp), %xmm0
 movups  92912(%rsp), %xmm1
 movaps  %xmm1, 94032(%rsp)
 movaps  %xmm0, 94016(%rsp)
 movaps  94016(%rsp), %xmm0
 movaps  94032(%rsp), %xmm1
 movaps  %xmm1, 92864(%rsp)
 movaps  %xmm0, 92848(%rsp)
 leaq    92632(%rsp), %rcx
 leaq    92848(%rsp), %rdx
 callq   jodin_rs::ast::jodin_node::JodinNode::into_inner
 jmp     .LBB7390_349
.LBB7390_344:
 leaq    92896(%rsp), %rdx
 leaq    93504(%rsp), %rcx
 movq    %rcx, 1944(%rsp)
 movl    $256, %r8d
 movq    %r8, 1952(%rsp)
 callq   memcpy
 movq    1944(%rsp), %rdx
 movq    1952(%rsp), %r8
 leaq    93760(%rsp), %rcx
 movq    %rcx, 1960(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1960(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_345
.LBB7390_345:
 jmp     .LBB7390_348
.LBB7390_346:
 jmp     .LBB7390_294
.LBB7390_347:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_346
.LBB7390_348:
 movb    $0, 117950(%rsp)
 jmp     .LBB7390_309
.LBB7390_349:
 movzbl  92632(%rsp), %eax
 cmpq    $20, %rax
 jne     .LBB7390_351
 movq    92640(%rsp), %rax
 movq    %rax, 94056(%rsp)
 movq    92648(%rsp), %rax
 movq    %rax, 94064(%rsp)
 movq    92656(%rsp), %rax
 movq    %rax, 94072(%rsp)
 movb    $1, 117950(%rsp)
 movq    94056(%rsp), %rax
 movq    %rax, 92608(%rsp)
 movq    94064(%rsp), %rax
 movq    %rax, 92616(%rsp)
 movq    94072(%rsp), %rax
 movq    %rax, 92624(%rsp)
 movzbl  92632(%rsp), %eax
 cmpq    $20, %rax
 je      .LBB7390_354
 jmp     .LBB7390_355
.LBB7390_351:
 leaq    .L__unnamed_372(%rip), %rcx
 leaq    .L__unnamed_375(%rip), %r8
 movl    $34, %edx
 callq   std::panicking::begin_panic
 jmp     .LBB7390_9
.LBB7390_352:
 leaq    92632(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_346
.LBB7390_353:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_352
.LBB7390_354:
 leaq    94920(%rsp), %rcx
 leaq    92032(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_359
.LBB7390_355:
 leaq    92632(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_354
.LBB7390_356:
 jmp     .LBB7390_358
.LBB7390_357:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_356
.LBB7390_358:
 testb   $1, 117950(%rsp)
 jne     .LBB7390_387
 jmp     .LBB7390_294
.LBB7390_359:
 leaq    .L__unnamed_376(%rip), %r8
 leaq    94888(%rsp), %rcx
 leaq    94920(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_361
.LBB7390_360:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_358
.LBB7390_361:
 movb    $1, 117949(%rsp)
 leaq    94952(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_362
.LBB7390_362:
 movq    2792(%rsp), %rdx
 movb    $0, 117949(%rsp)
 leaq    94624(%rsp), %rcx
 leaq    94888(%rsp), %r8
 leaq    94952(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_365
.LBB7390_363:
 testb   $1, 117949(%rsp)
 jne     .LBB7390_366
 jmp     .LBB7390_358
.LBB7390_364:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_363
.LBB7390_365:
 movb    $0, 117949(%rsp)
 leaq    94360(%rsp), %rcx
 leaq    94624(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_367
.LBB7390_366:
 leaq    94888(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_358
.LBB7390_367:
 movq    94360(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_369
 jmp     .LBB7390_1713
.LBB7390_1713:
 jmp     .LBB7390_370
 ud2
.LBB7390_369:
 movups  94368(%rsp), %xmm0
 movups  94384(%rsp), %xmm1
 movaps  %xmm1, 95504(%rsp)
 movaps  %xmm0, 95488(%rsp)
 movaps  95488(%rsp), %xmm0
 movaps  95504(%rsp), %xmm1
 movaps  %xmm1, 94336(%rsp)
 movaps  %xmm0, 94320(%rsp)
 leaq    94104(%rsp), %rcx
 leaq    94320(%rsp), %rdx
 callq   jodin_rs::ast::jodin_node::JodinNode::into_inner
 jmp     .LBB7390_374
.LBB7390_370:
 leaq    94368(%rsp), %rdx
 leaq    94976(%rsp), %rcx
 movq    %rcx, 1920(%rsp)
 movl    $256, %r8d
 movq    %r8, 1928(%rsp)
 callq   memcpy
 movq    1920(%rsp), %rdx
 movq    1928(%rsp), %r8
 leaq    95232(%rsp), %rcx
 movq    %rcx, 1936(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1936(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_371
.LBB7390_371:
 movb    $0, 117948(%rsp)
 leaq    92608(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_348
.LBB7390_372:
 jmp     .LBB7390_358
.LBB7390_373:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_372
.LBB7390_374:
 movzbl  94104(%rsp), %eax
 cmpq    $20, %rax
 jne     .LBB7390_376
 movq    94112(%rsp), %rax
 movq    %rax, 95520(%rsp)
 movq    94120(%rsp), %rax
 movq    %rax, 95528(%rsp)
 movq    94128(%rsp), %rax
 movq    %rax, 95536(%rsp)
 movb    $1, 117948(%rsp)
 movq    95520(%rsp), %rax
 movq    %rax, 94080(%rsp)
 movq    95528(%rsp), %rax
 movq    %rax, 94088(%rsp)
 movq    95536(%rsp), %rax
 movq    %rax, 94096(%rsp)
 movzbl  94104(%rsp), %eax
 cmpq    $20, %rax
 je      .LBB7390_379
 jmp     .LBB7390_380
.LBB7390_376:
 leaq    .L__unnamed_372(%rip), %rcx
 leaq    .L__unnamed_377(%rip), %r8
 movl    $34, %edx
 callq   std::panicking::begin_panic
 jmp     .LBB7390_9
.LBB7390_377:
 leaq    94104(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_372
.LBB7390_378:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_377
.LBB7390_379:
 movb    $0, 117915(%rsp)
 movups  92040(%rsp), %xmm0
 movups  92056(%rsp), %xmm1
 movaps  %xmm1, 95776(%rsp)
 movaps  %xmm0, 95760(%rsp)
 movb    $0, 117950(%rsp)
 movq    92624(%rsp), %rcx
 movq    %rcx, 95808(%rsp)
 movups  92608(%rsp), %xmm0
 movaps  %xmm0, 95792(%rsp)
 movb    $0, 117948(%rsp)
 movq    94096(%rsp), %rcx
 movq    %rcx, 95840(%rsp)
 movups  94080(%rsp), %xmm0
 movaps  %xmm0, 95824(%rsp)
 movaps  95760(%rsp), %xmm0
 movaps  95776(%rsp), %xmm1
 movups  %xmm1, 95568(%rsp)
 movups  %xmm0, 95552(%rsp)
 movq    95808(%rsp), %rcx
 movq    %rcx, 95600(%rsp)
 movaps  95792(%rsp), %xmm0
 movups  %xmm0, 95584(%rsp)
 movq    95840(%rsp), %rcx
 movq    %rcx, 95624(%rsp)
 movaps  95824(%rsp), %xmm0
 movups  %xmm0, 95608(%rsp)
 movb    $15, 95544(%rsp)
 leaq    92072(%rsp), %rcx
 leaq    95544(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_384
.LBB7390_380:
 leaq    94104(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_379
.LBB7390_381:
 jmp     .LBB7390_383
.LBB7390_382:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_381
.LBB7390_383:
 testb   $1, 117948(%rsp)
 jne     .LBB7390_386
 jmp     .LBB7390_358
.LBB7390_384:
 movb    $1, 117941(%rsp)
 movb    $0, 117948(%rsp)
 movb    $0, 117950(%rsp)
 jmp     .LBB7390_326
.LBB7390_385:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_383
.LBB7390_386:
 leaq    94080(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_358
.LBB7390_387:
 leaq    92608(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_294
.LBB7390_388:
 leaq    98232(%rsp), %rcx
 leaq    92032(%rsp), %rdx
 movl    $1, %r8d
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_389
.LBB7390_389:
 leaq    .L__unnamed_378(%rip), %r8
 leaq    98200(%rsp), %rcx
 leaq    98232(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_390
.LBB7390_390:
 movb    $1, 117945(%rsp)
 leaq    98264(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_391
.LBB7390_391:
 movq    2792(%rsp), %rdx
 movb    $0, 117945(%rsp)
 leaq    97936(%rsp), %rcx
 leaq    98200(%rsp), %r8
 leaq    98264(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_394
.LBB7390_392:
 testb   $1, 117945(%rsp)
 jne     .LBB7390_395
 jmp     .LBB7390_294
.LBB7390_393:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_392
.LBB7390_394:
 movb    $0, 117945(%rsp)
 leaq    97672(%rsp), %rcx
 leaq    97936(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_396
.LBB7390_395:
 leaq    98200(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_294
.LBB7390_396:
 movq    97672(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_398
 jmp     .LBB7390_1714
.LBB7390_1714:
 jmp     .LBB7390_399
 ud2
.LBB7390_398:
 movups  97680(%rsp), %xmm0
 movups  97696(%rsp), %xmm1
 movaps  %xmm1, 98816(%rsp)
 movaps  %xmm0, 98800(%rsp)
 movb    $1, 117944(%rsp)
 movaps  98800(%rsp), %xmm0
 movaps  98816(%rsp), %xmm1
 movaps  %xmm1, 97648(%rsp)
 movaps  %xmm0, 97632(%rsp)
 movb    $0, 117915(%rsp)
 movups  92040(%rsp), %xmm0
 movups  92056(%rsp), %xmm1
 movaps  %xmm1, 99104(%rsp)
 movaps  %xmm0, 99088(%rsp)
 movaps  99088(%rsp), %xmm0
 movaps  99104(%rsp), %xmm1
 movups  %xmm1, 98896(%rsp)
 movups  %xmm0, 98880(%rsp)
 movb    $28, 98872(%rsp)
 leaq    98840(%rsp), %rcx
 leaq    98872(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_403
.LBB7390_399:
 leaq    97680(%rsp), %rdx
 leaq    98288(%rsp), %rcx
 movq    %rcx, 1896(%rsp)
 movl    $256, %r8d
 movq    %r8, 1904(%rsp)
 callq   memcpy
 movq    1896(%rsp), %rdx
 movq    1904(%rsp), %r8
 leaq    98544(%rsp), %rcx
 movq    %rcx, 1912(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1912(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_400
.LBB7390_400:
 movb    $0, 117944(%rsp)
 jmp     .LBB7390_309
.LBB7390_401:
 jmp     .LBB7390_294
.LBB7390_402:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_401
.LBB7390_403:
 movups  98840(%rsp), %xmm0
 movups  98856(%rsp), %xmm1
 movaps  %xmm1, 99360(%rsp)
 movaps  %xmm0, 99344(%rsp)
 movb    $0, 117944(%rsp)
 movaps  97632(%rsp), %xmm0
 movaps  97648(%rsp), %xmm1
 movaps  %xmm1, 99392(%rsp)
 movaps  %xmm0, 99376(%rsp)
 movaps  99344(%rsp), %xmm0
 movaps  99360(%rsp), %xmm1
 movups  %xmm1, 99152(%rsp)
 movups  %xmm0, 99136(%rsp)
 movaps  99376(%rsp), %xmm0
 movaps  99392(%rsp), %xmm1
 movups  %xmm1, 99184(%rsp)
 movups  %xmm0, 99168(%rsp)
 movb    $16, 99128(%rsp)
 leaq    92072(%rsp), %rcx
 leaq    99128(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_406
.LBB7390_404:
 testb   $1, 117944(%rsp)
 jne     .LBB7390_407
 jmp     .LBB7390_294
.LBB7390_405:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_404
.LBB7390_406:
 movb    $1, 117941(%rsp)
 movb    $0, 117944(%rsp)
 jmp     .LBB7390_326
.LBB7390_407:
 leaq    97632(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_294
.LBB7390_408:
 leaq    100008(%rsp), %rcx
 leaq    92032(%rsp), %rdx
 movl    $1, %r8d
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_409
.LBB7390_409:
 leaq    .L__unnamed_379(%rip), %r8
 leaq    99976(%rsp), %rcx
 leaq    100008(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_410
.LBB7390_410:
 movb    $1, 117943(%rsp)
 leaq    100040(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_411
.LBB7390_411:
 movq    2792(%rsp), %rdx
 movb    $0, 117943(%rsp)
 leaq    99712(%rsp), %rcx
 leaq    99976(%rsp), %r8
 leaq    100040(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_414
.LBB7390_412:
 testb   $1, 117943(%rsp)
 jne     .LBB7390_415
 jmp     .LBB7390_294
.LBB7390_413:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_412
.LBB7390_414:
 movb    $0, 117943(%rsp)
 leaq    99448(%rsp), %rcx
 leaq    99712(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_416
.LBB7390_415:
 leaq    99976(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_294
.LBB7390_416:
 movq    99448(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_418
 jmp     .LBB7390_1715
.LBB7390_1715:
 jmp     .LBB7390_419
 ud2
.LBB7390_418:
 movups  99456(%rsp), %xmm0
 movups  99472(%rsp), %xmm1
 movaps  %xmm1, 100592(%rsp)
 movaps  %xmm0, 100576(%rsp)
 movb    $1, 117942(%rsp)
 movaps  100576(%rsp), %xmm0
 movaps  100592(%rsp), %xmm1
 movaps  %xmm1, 99424(%rsp)
 movaps  %xmm0, 99408(%rsp)
 movb    $0, 117915(%rsp)
 movups  92040(%rsp), %xmm0
 movups  92056(%rsp), %xmm1
 movaps  %xmm1, 100848(%rsp)
 movaps  %xmm0, 100832(%rsp)
 movb    $0, 117942(%rsp)
 movaps  99408(%rsp), %xmm0
 movaps  99424(%rsp), %xmm1
 movaps  %xmm1, 100880(%rsp)
 movaps  %xmm0, 100864(%rsp)
 movaps  100832(%rsp), %xmm0
 movaps  100848(%rsp), %xmm1
 movups  %xmm1, 100640(%rsp)
 movups  %xmm0, 100624(%rsp)
 movaps  100864(%rsp), %xmm0
 movaps  100880(%rsp), %xmm1
 movups  %xmm1, 100672(%rsp)
 movups  %xmm0, 100656(%rsp)
 movb    $16, 100616(%rsp)
 leaq    92072(%rsp), %rcx
 leaq    100616(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_423
.LBB7390_419:
 leaq    99456(%rsp), %rdx
 leaq    100064(%rsp), %rcx
 movq    %rcx, 1872(%rsp)
 movl    $256, %r8d
 movq    %r8, 1880(%rsp)
 callq   memcpy
 movq    1872(%rsp), %rdx
 movq    1880(%rsp), %r8
 leaq    100320(%rsp), %rcx
 movq    %rcx, 1888(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1888(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_420
.LBB7390_420:
 movb    $0, 117942(%rsp)
 jmp     .LBB7390_309
.LBB7390_421:
 jmp     .LBB7390_294
.LBB7390_422:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_421
.LBB7390_423:
 movb    $1, 117941(%rsp)
 movb    $0, 117942(%rsp)
 jmp     .LBB7390_326
.LBB7390_424:
 testb   $1, 117942(%rsp)
 jne     .LBB7390_426
 jmp     .LBB7390_294
.LBB7390_425:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_424
.LBB7390_426:
 leaq    99408(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_294
.LBB7390_427:
 movq    100976(%rsp), %rax
 testq   %rax, %rax
 setne   %al
 movzbl  %al, %eax
 je      .LBB7390_431
 jmp     .LBB7390_1719
.LBB7390_1719:
 jmp     .LBB7390_432
.LBB7390_428:
 testb   $1, 117941(%rsp)
 jne     .LBB7390_447
 jmp     .LBB7390_294
.LBB7390_429:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_428
 ud2
.LBB7390_431:
 movb    $0, 117941(%rsp)
 movq    92072(%rsp), %rax
 movq    %rax, 3376(%rsp)
 movq    92080(%rsp), %rax
 movq    %rax, 3384(%rsp)
 movq    92088(%rsp), %rax
 movq    %rax, 3392(%rsp)
 movq    92096(%rsp), %rax
 movq    %rax, 3400(%rsp)
 jmp     .LBB7390_450
.LBB7390_432:
 movups  100976(%rsp), %xmm0
 movups  100992(%rsp), %xmm1
 movaps  %xmm1, 101024(%rsp)
 movaps  %xmm0, 101008(%rsp)
 movb    $1, 117940(%rsp)
 movaps  101008(%rsp), %xmm0
 movaps  101024(%rsp), %xmm1
 movaps  %xmm1, 101584(%rsp)
 movaps  %xmm0, 101568(%rsp)
 movl    $32, %ecx
 movl    $8, %edx
 callq   alloc::alloc::exchange_malloc
 movq    %rax, %rdx
 movb    $0, 117941(%rsp)
 movups  92072(%rsp), %xmm0
 movups  92088(%rsp), %xmm1
 movaps  %xmm1, 101648(%rsp)
 movaps  %xmm0, 101632(%rsp)
 movaps  101632(%rsp), %xmm0
 movaps  101648(%rsp), %xmm1
 movups  %xmm1, 16(%rdx)
 movups  %xmm0, (%rdx)
 leaq    101608(%rsp), %rcx
 movl    $1, %r8d
 callq   alloc::slice::<impl [T]>::into_vec
 jmp     .LBB7390_433
.LBB7390_433:
 movq    2792(%rsp), %rdx
 movb    $0, 117940(%rsp)
 leaq    101304(%rsp), %rcx
 leaq    101568(%rsp), %r8
 leaq    101608(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_436
.LBB7390_434:
 testb   $1, 117940(%rsp)
 jne     .LBB7390_438
 jmp     .LBB7390_437
.LBB7390_435:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_434
.LBB7390_436:
 movb    $0, 117940(%rsp)
 leaq    101040(%rsp), %rcx
 leaq    101304(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_439
.LBB7390_437:
 jmp     .LBB7390_428
.LBB7390_438:
 leaq    101568(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_437
.LBB7390_439:
 movq    101040(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_442
 jmp     .LBB7390_1720
.LBB7390_1720:
 jmp     .LBB7390_443
.LBB7390_440:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_437
 ud2
.LBB7390_442:
 movq    101048(%rsp), %rax
 movq    %rax, 102176(%rsp)
 movq    101056(%rsp), %rax
 movq    %rax, 102184(%rsp)
 movq    101064(%rsp), %rax
 movq    %rax, 102192(%rsp)
 movq    101072(%rsp), %rax
 movq    %rax, 102200(%rsp)
 movq    102176(%rsp), %rax
 movq    %rax, 3376(%rsp)
 movq    102184(%rsp), %rax
 movq    %rax, 3384(%rsp)
 movq    102192(%rsp), %rax
 movq    %rax, 3392(%rsp)
 movq    102200(%rsp), %rax
 movq    %rax, 3400(%rsp)
 jmp     .LBB7390_450
.LBB7390_443:
 leaq    101048(%rsp), %rdx
 leaq    101664(%rsp), %rcx
 movq    %rcx, 1848(%rsp)
 movl    $256, %r8d
 movq    %r8, 1856(%rsp)
 callq   memcpy
 movq    1848(%rsp), %rdx
 movq    1856(%rsp), %r8
 leaq    101920(%rsp), %rcx
 movq    %rcx, 1864(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1864(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_444
.LBB7390_444:
 jmp     .LBB7390_309
.LBB7390_445:
 jmp     .LBB7390_437
.LBB7390_446:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_445
.LBB7390_447:
 leaq    92072(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_294
.LBB7390_448:
 movb    $0, 117915(%rsp)
 leaq    92032(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_104
.LBB7390_449:
 leaq    92040(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_448
.LBB7390_450:
 movb    $0, 117941(%rsp)
 movb    $0, 117915(%rsp)
 leaq    92032(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_89
.LBB7390_451:
 movq    2000(%rsp), %rcx
 movq    2008(%rsp), %rdx
 movq    %rdx, 100952(%rsp)
 movq    %rcx, 100960(%rsp)
 movq    %rsp, %rcx
 movq    $1, 32(%rcx)
 leaq    .L__unnamed_380(%rip), %rdx
 leaq    100904(%rsp), %rcx
 movl    $1, %r8d
 leaq    100952(%rsp), %r9
 callq   core::fmt::Arguments::new_v1
 jmp     .LBB7390_452
.LBB7390_452:
 leaq    .L__unnamed_381(%rip), %rdx
 leaq    100904(%rsp), %rcx
 callq   std::panicking::begin_panic_fmt
 jmp     .LBB7390_9
.LBB7390_453:
 leaq    92040(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_285
.LBB7390_454:
 cmpq    $1, 2848(%rsp)
 jae     .LBB7390_456
.LBB7390_455:
 leaq    2840(%rsp), %rax
 movq    %rax, 90552(%rsp)
 movq    90552(%rsp), %rcx
 movq    %rcx, 118160(%rsp)
 leaq    _ZN67_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17he8271684dacd979dE(%rip), %rdx
 callq   core::fmt::ArgumentV1::new
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 1832(%rsp)
 movq    %rcx, 1840(%rsp)
 jmp     .LBB7390_695
.LBB7390_456:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 movq    %rax, 1824(%rsp)
 subq    $30, %rax
 je      .LBB7390_458
 jmp     .LBB7390_1721
.LBB7390_1721:
 movq    1824(%rsp), %rax
 subq    $99, %rax
 je      .LBB7390_459
 jmp     .LBB7390_1722
.LBB7390_1722:
 movq    1824(%rsp), %rax
 subq    $137, %rax
 je      .LBB7390_460
 jmp     .LBB7390_457
.LBB7390_457:
 cmpq    $4, 2848(%rsp)
 jae     .LBB7390_462
 jmp     .LBB7390_461
.LBB7390_458:
 movb    $25, 77256(%rsp)
 leaq    72784(%rsp), %rcx
 leaq    77256(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_664
.LBB7390_459:
 movb    $99, 76143(%rsp)
 movb    76143(%rsp), %r8b
 leaq    75872(%rsp), %rcx
 leaq    72688(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_643
.LBB7390_460:
 movb    $-119, 73919(%rsp)
 movb    73919(%rsp), %r8b
 leaq    73648(%rsp), %rcx
 leaq    72688(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_622
.LBB7390_461:
 cmpq    $3, 2848(%rsp)
 jae     .LBB7390_560
 jmp     .LBB7390_455
.LBB7390_462:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 cmpq    $29, %rax
 jne     .LBB7390_461
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $170, %rax
 jne     .LBB7390_461
 movq    2840(%rsp), %rax
 movzbl  2(%rax), %eax
 cmpq    $159, %rax
 jne     .LBB7390_461
 movq    2840(%rsp), %rax
 movzbl  3(%rax), %eax
 cmpq    $156, %rax
 jne     .LBB7390_461
 movb    $-86, 78671(%rsp)
 movb    78671(%rsp), %r8b
 leaq    78400(%rsp), %rcx
 leaq    72688(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_467
.LBB7390_467:
 leaq    78136(%rsp), %rcx
 leaq    78400(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_470
.LBB7390_468:
 leaq    72688(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_85
.LBB7390_469:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_468
.LBB7390_470:
 movq    78136(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_472
 jmp     .LBB7390_1731
.LBB7390_1731:
 jmp     .LBB7390_473
 ud2
.LBB7390_472:
 movq    2792(%rsp), %rdx
 movups  78144(%rsp), %xmm0
 movups  78160(%rsp), %xmm1
 movaps  %xmm1, 79200(%rsp)
 movaps  %xmm0, 79184(%rsp)
 movaps  79184(%rsp), %xmm0
 movaps  79200(%rsp), %xmm1
 movaps  %xmm1, 78112(%rsp)
 movaps  %xmm0, 78096(%rsp)
 leaq    77832(%rsp), %rcx
 leaq    78096(%rsp), %r8
 callq   jodin_rs::ast::JodinNodeGenerator::new_intermediate_type
 jmp     .LBB7390_478
.LBB7390_473:
 leaq    78144(%rsp), %rdx
 leaq    78672(%rsp), %rcx
 movq    %rcx, 1800(%rsp)
 movl    $256, %r8d
 movq    %r8, 1808(%rsp)
 callq   memcpy
 movq    1800(%rsp), %rdx
 movq    1808(%rsp), %r8
 leaq    78928(%rsp), %rcx
 movq    %rcx, 1816(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1816(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_474
.LBB7390_474:
 jmp     .LBB7390_477
.LBB7390_475:
 jmp     .LBB7390_468
.LBB7390_476:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_475
.LBB7390_477:
 jmp     .LBB7390_486
.LBB7390_478:
 leaq    77568(%rsp), %rcx
 leaq    77832(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_479
.LBB7390_479:
 movq    77568(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_481
 jmp     .LBB7390_1732
.LBB7390_1732:
 jmp     .LBB7390_482
 ud2
.LBB7390_481:
 movups  77656(%rsp), %xmm0
 movaps  %xmm0, 79808(%rsp)
 movups  77640(%rsp), %xmm0
 movaps  %xmm0, 79792(%rsp)
 movups  77576(%rsp), %xmm0
 movups  77592(%rsp), %xmm1
 movups  77608(%rsp), %xmm2
 movups  77624(%rsp), %xmm3
 movaps  %xmm3, 79776(%rsp)
 movaps  %xmm2, 79760(%rsp)
 movaps  %xmm1, 79744(%rsp)
 movaps  %xmm0, 79728(%rsp)
 movb    $1, 117960(%rsp)
 movaps  79808(%rsp), %xmm0
 movaps  %xmm0, 77552(%rsp)
 movaps  79792(%rsp), %xmm0
 movaps  %xmm0, 77536(%rsp)
 movaps  79728(%rsp), %xmm0
 movaps  79744(%rsp), %xmm1
 movaps  79760(%rsp), %xmm2
 movaps  79776(%rsp), %xmm3
 movaps  %xmm3, 77520(%rsp)
 movaps  %xmm2, 77504(%rsp)
 movaps  %xmm1, 77488(%rsp)
 movaps  %xmm0, 77472(%rsp)
 movb    $-97, 81199(%rsp)
 movb    81199(%rsp), %r8b
 leaq    80928(%rsp), %rcx
 leaq    72688(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_487
.LBB7390_482:
 leaq    77576(%rsp), %rdx
 leaq    79216(%rsp), %rcx
 movq    %rcx, 1776(%rsp)
 movl    $256, %r8d
 movq    %r8, 1784(%rsp)
 callq   memcpy
 movq    1776(%rsp), %rdx
 movq    1784(%rsp), %r8
 leaq    79472(%rsp), %rcx
 movq    %rcx, 1792(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1792(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_483
.LBB7390_483:
 jmp     .LBB7390_477
.LBB7390_484:
 jmp     .LBB7390_475
.LBB7390_485:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_484
.LBB7390_486:
 movb    $0, 117960(%rsp)
 jmp     .LBB7390_543
.LBB7390_487:
 leaq    80664(%rsp), %rcx
 leaq    80928(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_490
.LBB7390_488:
 testb   $1, 117960(%rsp)
 jne     .LBB7390_558
 jmp     .LBB7390_468
.LBB7390_489:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_488
.LBB7390_490:
 movq    80664(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_492
 jmp     .LBB7390_1733
.LBB7390_1733:
 jmp     .LBB7390_493
 ud2
.LBB7390_492:
 movups  80672(%rsp), %xmm0
 movups  80688(%rsp), %xmm1
 movaps  %xmm1, 81728(%rsp)
 movaps  %xmm0, 81712(%rsp)
 movb    $1, 117959(%rsp)
 movaps  81712(%rsp), %xmm0
 movaps  81728(%rsp), %xmm1
 movaps  %xmm1, 80640(%rsp)
 movaps  %xmm0, 80624(%rsp)
 leaq    81752(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_498
.LBB7390_493:
 leaq    80672(%rsp), %rdx
 leaq    81200(%rsp), %rcx
 movq    %rcx, 1752(%rsp)
 movl    $256, %r8d
 movq    %r8, 1760(%rsp)
 callq   memcpy
 movq    1752(%rsp), %rdx
 movq    1760(%rsp), %r8
 leaq    81456(%rsp), %rcx
 movq    %rcx, 1768(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1768(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_494
.LBB7390_494:
 movb    $0, 117959(%rsp)
 jmp     .LBB7390_497
.LBB7390_495:
 jmp     .LBB7390_488
.LBB7390_496:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_495
.LBB7390_497:
 jmp     .LBB7390_510
.LBB7390_498:
 movq    2792(%rsp), %rdx
 movb    $0, 117959(%rsp)
 leaq    80360(%rsp), %rcx
 leaq    80624(%rsp), %r8
 leaq    81752(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_501
.LBB7390_499:
 testb   $1, 117959(%rsp)
 jne     .LBB7390_502
 jmp     .LBB7390_495
.LBB7390_500:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_499
.LBB7390_501:
 movb    $0, 117959(%rsp)
 leaq    80096(%rsp), %rcx
 leaq    80360(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_503
.LBB7390_502:
 leaq    80624(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_495
.LBB7390_503:
 movq    80096(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_505
 jmp     .LBB7390_1734
.LBB7390_1734:
 jmp     .LBB7390_506
 ud2
.LBB7390_505:
 movups  80104(%rsp), %xmm0
 movups  80120(%rsp), %xmm1
 movaps  %xmm1, 82304(%rsp)
 movaps  %xmm0, 82288(%rsp)
 movaps  82288(%rsp), %xmm0
 movaps  82304(%rsp), %xmm1
 movaps  %xmm1, 80080(%rsp)
 movaps  %xmm0, 80064(%rsp)
 leaq    79848(%rsp), %rcx
 leaq    80064(%rsp), %rdx
 callq   jodin_rs::ast::jodin_node::JodinNode::into_inner
 jmp     .LBB7390_511
.LBB7390_506:
 leaq    80104(%rsp), %rdx
 leaq    81776(%rsp), %rcx
 movq    %rcx, 1728(%rsp)
 movl    $256, %r8d
 movq    %r8, 1736(%rsp)
 callq   memcpy
 movq    1728(%rsp), %rdx
 movq    1736(%rsp), %r8
 leaq    82032(%rsp), %rcx
 movq    %rcx, 1744(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1744(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_507
.LBB7390_507:
 jmp     .LBB7390_497
.LBB7390_508:
 jmp     .LBB7390_495
.LBB7390_509:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_508
.LBB7390_510:
 movb    $0, 117958(%rsp)
 leaq    77472(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::intermediate_type::IntermediateType>
 jmp     .LBB7390_486
.LBB7390_511:
 movzbl  79848(%rsp), %eax
 cmpq    $20, %rax
 jne     .LBB7390_513
 movq    79856(%rsp), %rax
 movq    %rax, 82328(%rsp)
 movq    79864(%rsp), %rax
 movq    %rax, 82336(%rsp)
 movq    79872(%rsp), %rax
 movq    %rax, 82344(%rsp)
 movb    $1, 117958(%rsp)
 movq    82328(%rsp), %rax
 movq    %rax, 79824(%rsp)
 movq    82336(%rsp), %rax
 movq    %rax, 79832(%rsp)
 movq    82344(%rsp), %rax
 movq    %rax, 79840(%rsp)
 movzbl  79848(%rsp), %eax
 cmpq    $20, %rax
 je      .LBB7390_516
 jmp     .LBB7390_517
.LBB7390_513:
 leaq    .L__unnamed_372(%rip), %rcx
 leaq    .L__unnamed_382(%rip), %r8
 movl    $34, %edx
 callq   std::panicking::begin_panic
 jmp     .LBB7390_9
.LBB7390_514:
 leaq    79848(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_508
.LBB7390_515:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_514
.LBB7390_516:
 movb    $-100, 83727(%rsp)
 movb    83727(%rsp), %r8b
 leaq    83456(%rsp), %rcx
 leaq    72688(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_521
.LBB7390_517:
 leaq    79848(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_516
.LBB7390_518:
 jmp     .LBB7390_520
.LBB7390_519:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_518
.LBB7390_520:
 testb   $1, 117958(%rsp)
 jne     .LBB7390_557
 jmp     .LBB7390_488
.LBB7390_521:
 leaq    83192(%rsp), %rcx
 leaq    83456(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_523
.LBB7390_522:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_520
.LBB7390_523:
 movq    83192(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_525
 jmp     .LBB7390_1735
.LBB7390_1735:
 jmp     .LBB7390_526
 ud2
.LBB7390_525:
 movups  83200(%rsp), %xmm0
 movups  83216(%rsp), %xmm1
 movaps  %xmm1, 84256(%rsp)
 movaps  %xmm0, 84240(%rsp)
 movb    $1, 117957(%rsp)
 movaps  84240(%rsp), %xmm0
 movaps  84256(%rsp), %xmm1
 movaps  %xmm1, 83168(%rsp)
 movaps  %xmm0, 83152(%rsp)
 leaq    84280(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_531
.LBB7390_526:
 leaq    83200(%rsp), %rdx
 leaq    83728(%rsp), %rcx
 movq    %rcx, 1704(%rsp)
 movl    $256, %r8d
 movq    %r8, 1712(%rsp)
 callq   memcpy
 movq    1704(%rsp), %rdx
 movq    1712(%rsp), %r8
 leaq    83984(%rsp), %rcx
 movq    %rcx, 1720(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1720(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_527
.LBB7390_527:
 movb    $0, 117957(%rsp)
 jmp     .LBB7390_530
.LBB7390_528:
 jmp     .LBB7390_520
.LBB7390_529:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_528
.LBB7390_530:
 movb    $0, 117956(%rsp)
 leaq    79824(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_510
.LBB7390_531:
 movq    2792(%rsp), %rdx
 movb    $0, 117957(%rsp)
 leaq    82888(%rsp), %rcx
 leaq    83152(%rsp), %r8
 leaq    84280(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_534
.LBB7390_532:
 testb   $1, 117957(%rsp)
 jne     .LBB7390_535
 jmp     .LBB7390_528
.LBB7390_533:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_532
.LBB7390_534:
 movb    $0, 117957(%rsp)
 leaq    82624(%rsp), %rcx
 leaq    82888(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_536
.LBB7390_535:
 leaq    83152(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_528
.LBB7390_536:
 movq    82624(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_538
 jmp     .LBB7390_1736
.LBB7390_1736:
 jmp     .LBB7390_539
 ud2
.LBB7390_538:
 movups  82632(%rsp), %xmm0
 movups  82648(%rsp), %xmm1
 movaps  %xmm1, 84832(%rsp)
 movaps  %xmm0, 84816(%rsp)
 movaps  84816(%rsp), %xmm0
 movaps  84832(%rsp), %xmm1
 movaps  %xmm1, 82608(%rsp)
 movaps  %xmm0, 82592(%rsp)
 leaq    82376(%rsp), %rcx
 leaq    82592(%rsp), %rdx
 callq   jodin_rs::ast::jodin_node::JodinNode::into_inner
 jmp     .LBB7390_544
.LBB7390_539:
 leaq    82632(%rsp), %rdx
 leaq    84304(%rsp), %rcx
 movq    %rcx, 1680(%rsp)
 movl    $256, %r8d
 movq    %r8, 1688(%rsp)
 callq   memcpy
 movq    1680(%rsp), %rdx
 movq    1688(%rsp), %r8
 leaq    84560(%rsp), %rcx
 movq    %rcx, 1696(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1696(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_540
.LBB7390_540:
 jmp     .LBB7390_530
.LBB7390_541:
 jmp     .LBB7390_528
.LBB7390_542:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_541
.LBB7390_543:
 leaq    72688(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_104
.LBB7390_544:
 movzbl  82376(%rsp), %eax
 cmpq    $20, %rax
 jne     .LBB7390_546
 movq    82384(%rsp), %rax
 movq    %rax, 84848(%rsp)
 movq    82392(%rsp), %rax
 movq    %rax, 84856(%rsp)
 movq    82400(%rsp), %rax
 movq    %rax, 84864(%rsp)
 movb    $1, 117956(%rsp)
 movq    84848(%rsp), %rax
 movq    %rax, 82352(%rsp)
 movq    84856(%rsp), %rax
 movq    %rax, 82360(%rsp)
 movq    84864(%rsp), %rax
 movq    %rax, 82368(%rsp)
 movzbl  82376(%rsp), %eax
 cmpq    $20, %rax
 je      .LBB7390_549
 jmp     .LBB7390_550
.LBB7390_546:
 leaq    .L__unnamed_372(%rip), %rcx
 leaq    .L__unnamed_383(%rip), %r8
 movl    $34, %edx
 callq   std::panicking::begin_panic
 jmp     .LBB7390_9
.LBB7390_547:
 leaq    82376(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_541
.LBB7390_548:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_547
.LBB7390_549:
 movb    $0, 117960(%rsp)
 movaps  77552(%rsp), %xmm0
 movaps  %xmm0, 85168(%rsp)
 movaps  77536(%rsp), %xmm0
 movaps  %xmm0, 85152(%rsp)
 movaps  77472(%rsp), %xmm0
 movaps  77488(%rsp), %xmm1
 movaps  77504(%rsp), %xmm2
 movaps  77520(%rsp), %xmm3
 movaps  %xmm3, 85136(%rsp)
 movaps  %xmm2, 85120(%rsp)
 movaps  %xmm1, 85104(%rsp)
 movaps  %xmm0, 85088(%rsp)
 movb    $0, 117958(%rsp)
 movq    79840(%rsp), %rcx
 movq    %rcx, 85200(%rsp)
 movups  79824(%rsp), %xmm0
 movaps  %xmm0, 85184(%rsp)
 movb    $0, 117956(%rsp)
 movq    82368(%rsp), %rcx
 movq    %rcx, 85232(%rsp)
 movups  82352(%rsp), %xmm0
 movaps  %xmm0, 85216(%rsp)
 movaps  85168(%rsp), %xmm0
 movups  %xmm0, 84960(%rsp)
 movaps  85152(%rsp), %xmm0
 movups  %xmm0, 84944(%rsp)
 movaps  85088(%rsp), %xmm0
 movaps  85104(%rsp), %xmm1
 movaps  85120(%rsp), %xmm2
 movaps  85136(%rsp), %xmm3
 movups  %xmm3, 84928(%rsp)
 movups  %xmm2, 84912(%rsp)
 movups  %xmm1, 84896(%rsp)
 movups  %xmm0, 84880(%rsp)
 movq    85200(%rsp), %rcx
 movq    %rcx, 84992(%rsp)
 movaps  85184(%rsp), %xmm0
 movups  %xmm0, 84976(%rsp)
 movq    85232(%rsp), %rcx
 movq    %rcx, 85016(%rsp)
 movaps  85216(%rsp), %xmm0
 movups  %xmm0, 85000(%rsp)
 movb    $26, 84872(%rsp)
 leaq    72784(%rsp), %rcx
 leaq    84872(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_554
.LBB7390_550:
 leaq    82376(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_549
.LBB7390_551:
 jmp     .LBB7390_553
.LBB7390_552:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_551
.LBB7390_553:
 testb   $1, 117956(%rsp)
 jne     .LBB7390_556
 jmp     .LBB7390_520
.LBB7390_554:
 movb    $0, 117956(%rsp)
 movb    $0, 117958(%rsp)
 movb    $0, 117960(%rsp)
 jmp     .LBB7390_559
.LBB7390_555:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_553
.LBB7390_556:
 leaq    82352(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_520
.LBB7390_557:
 leaq    79824(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_488
.LBB7390_558:
 leaq    77472(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::intermediate_type::IntermediateType>
 jmp     .LBB7390_468
.LBB7390_559:
 movb    $-101, 90831(%rsp)
 movb    90831(%rsp), %r8b
 leaq    90560(%rsp), %rcx
 leaq    72688(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_665
.LBB7390_560:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 cmpq    $29, %rax
 jne     .LBB7390_455
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $170, %rax
 jne     .LBB7390_455
 movq    2840(%rsp), %rax
 movzbl  2(%rax), %eax
 cmpq    $156, %rax
 jne     .LBB7390_455
 movb    $-86, 86447(%rsp)
 movb    86447(%rsp), %r8b
 leaq    86176(%rsp), %rcx
 leaq    72688(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_564
.LBB7390_564:
 leaq    85912(%rsp), %rcx
 leaq    86176(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_565
.LBB7390_565:
 movq    85912(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_567
 jmp     .LBB7390_1727
.LBB7390_1727:
 jmp     .LBB7390_568
 ud2
.LBB7390_567:
 movq    2792(%rsp), %rdx
 movups  85920(%rsp), %xmm0
 movups  85936(%rsp), %xmm1
 movaps  %xmm1, 86976(%rsp)
 movaps  %xmm0, 86960(%rsp)
 movaps  86960(%rsp), %xmm0
 movaps  86976(%rsp), %xmm1
 movaps  %xmm1, 85888(%rsp)
 movaps  %xmm0, 85872(%rsp)
 leaq    85608(%rsp), %rcx
 leaq    85872(%rsp), %r8
 callq   jodin_rs::ast::JodinNodeGenerator::new_intermediate_type
 jmp     .LBB7390_573
.LBB7390_568:
 leaq    85920(%rsp), %rdx
 leaq    86448(%rsp), %rcx
 movq    %rcx, 1656(%rsp)
 movl    $256, %r8d
 movq    %r8, 1664(%rsp)
 callq   memcpy
 movq    1656(%rsp), %rdx
 movq    1664(%rsp), %r8
 leaq    86704(%rsp), %rcx
 movq    %rcx, 1672(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1672(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_569
.LBB7390_569:
 jmp     .LBB7390_572
.LBB7390_570:
 jmp     .LBB7390_468
.LBB7390_571:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_570
.LBB7390_572:
 jmp     .LBB7390_581
.LBB7390_573:
 leaq    85344(%rsp), %rcx
 leaq    85608(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_574
.LBB7390_574:
 movq    85344(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_576
 jmp     .LBB7390_1728
.LBB7390_1728:
 jmp     .LBB7390_577
 ud2
.LBB7390_576:
 movups  85432(%rsp), %xmm0
 movaps  %xmm0, 87584(%rsp)
 movups  85416(%rsp), %xmm0
 movaps  %xmm0, 87568(%rsp)
 movups  85352(%rsp), %xmm0
 movups  85368(%rsp), %xmm1
 movups  85384(%rsp), %xmm2
 movups  85400(%rsp), %xmm3
 movaps  %xmm3, 87552(%rsp)
 movaps  %xmm2, 87536(%rsp)
 movaps  %xmm1, 87520(%rsp)
 movaps  %xmm0, 87504(%rsp)
 movb    $1, 117955(%rsp)
 movaps  87584(%rsp), %xmm0
 movaps  %xmm0, 85328(%rsp)
 movaps  87568(%rsp), %xmm0
 movaps  %xmm0, 85312(%rsp)
 movaps  87504(%rsp), %xmm0
 movaps  87520(%rsp), %xmm1
 movaps  87536(%rsp), %xmm2
 movaps  87552(%rsp), %xmm3
 movaps  %xmm3, 85296(%rsp)
 movaps  %xmm2, 85280(%rsp)
 movaps  %xmm1, 85264(%rsp)
 movaps  %xmm0, 85248(%rsp)
 movb    $-100, 88975(%rsp)
 movb    88975(%rsp), %r8b
 leaq    88704(%rsp), %rcx
 leaq    72688(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_582
.LBB7390_577:
 leaq    85352(%rsp), %rdx
 leaq    86992(%rsp), %rcx
 movq    %rcx, 1632(%rsp)
 movl    $256, %r8d
 movq    %r8, 1640(%rsp)
 callq   memcpy
 movq    1632(%rsp), %rdx
 movq    1640(%rsp), %r8
 leaq    87248(%rsp), %rcx
 movq    %rcx, 1648(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1648(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_578
.LBB7390_578:
 jmp     .LBB7390_572
.LBB7390_579:
 jmp     .LBB7390_570
.LBB7390_580:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_579
.LBB7390_581:
 movb    $0, 117955(%rsp)
 jmp     .LBB7390_543
.LBB7390_582:
 leaq    88440(%rsp), %rcx
 leaq    88704(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_585
.LBB7390_583:
 testb   $1, 117955(%rsp)
 jne     .LBB7390_621
 jmp     .LBB7390_468
.LBB7390_584:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_583
.LBB7390_585:
 movq    88440(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_587
 jmp     .LBB7390_1729
.LBB7390_1729:
 jmp     .LBB7390_588
 ud2
.LBB7390_587:
 movups  88448(%rsp), %xmm0
 movups  88464(%rsp), %xmm1
 movaps  %xmm1, 89504(%rsp)
 movaps  %xmm0, 89488(%rsp)
 movb    $1, 117954(%rsp)
 movaps  89488(%rsp), %xmm0
 movaps  89504(%rsp), %xmm1
 movaps  %xmm1, 88416(%rsp)
 movaps  %xmm0, 88400(%rsp)
 leaq    89528(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_593
.LBB7390_588:
 leaq    88448(%rsp), %rdx
 leaq    88976(%rsp), %rcx
 movq    %rcx, 1608(%rsp)
 movl    $256, %r8d
 movq    %r8, 1616(%rsp)
 callq   memcpy
 movq    1608(%rsp), %rdx
 movq    1616(%rsp), %r8
 leaq    89232(%rsp), %rcx
 movq    %rcx, 1624(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1624(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_589
.LBB7390_589:
 movb    $0, 117954(%rsp)
 jmp     .LBB7390_592
.LBB7390_590:
 jmp     .LBB7390_583
.LBB7390_591:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_590
.LBB7390_592:
 movb    $0, 117953(%rsp)
 leaq    85248(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::intermediate_type::IntermediateType>
 jmp     .LBB7390_581
.LBB7390_593:
 movq    2792(%rsp), %rdx
 movb    $0, 117954(%rsp)
 leaq    88136(%rsp), %rcx
 leaq    88400(%rsp), %r8
 leaq    89528(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_596
.LBB7390_594:
 testb   $1, 117954(%rsp)
 jne     .LBB7390_597
 jmp     .LBB7390_590
.LBB7390_595:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_594
.LBB7390_596:
 movb    $0, 117954(%rsp)
 leaq    87872(%rsp), %rcx
 leaq    88136(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_598
.LBB7390_597:
 leaq    88400(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_590
.LBB7390_598:
 movq    87872(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_600
 jmp     .LBB7390_1730
.LBB7390_1730:
 jmp     .LBB7390_601
 ud2
.LBB7390_600:
 movups  87880(%rsp), %xmm0
 movups  87896(%rsp), %xmm1
 movaps  %xmm1, 90080(%rsp)
 movaps  %xmm0, 90064(%rsp)
 movaps  90064(%rsp), %xmm0
 movaps  90080(%rsp), %xmm1
 movaps  %xmm1, 87856(%rsp)
 movaps  %xmm0, 87840(%rsp)
 leaq    87624(%rsp), %rcx
 leaq    87840(%rsp), %rdx
 callq   jodin_rs::ast::jodin_node::JodinNode::into_inner
 jmp     .LBB7390_605
.LBB7390_601:
 leaq    87880(%rsp), %rdx
 leaq    89552(%rsp), %rcx
 movq    %rcx, 1584(%rsp)
 movl    $256, %r8d
 movq    %r8, 1592(%rsp)
 callq   memcpy
 movq    1584(%rsp), %rdx
 movq    1592(%rsp), %r8
 leaq    89808(%rsp), %rcx
 movq    %rcx, 1600(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1600(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_602
.LBB7390_602:
 jmp     .LBB7390_592
.LBB7390_603:
 jmp     .LBB7390_590
.LBB7390_604:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_603
.LBB7390_605:
 movzbl  87624(%rsp), %eax
 cmpq    $20, %rax
 jne     .LBB7390_607
 movq    87632(%rsp), %rax
 movq    %rax, 90096(%rsp)
 movq    87640(%rsp), %rax
 movq    %rax, 90104(%rsp)
 movq    87648(%rsp), %rax
 movq    %rax, 90112(%rsp)
 movb    $1, 117953(%rsp)
 movq    90096(%rsp), %rax
 movq    %rax, 87600(%rsp)
 movq    90104(%rsp), %rax
 movq    %rax, 87608(%rsp)
 movq    90112(%rsp), %rax
 movq    %rax, 87616(%rsp)
 movzbl  87624(%rsp), %eax
 cmpq    $20, %rax
 je      .LBB7390_610
 jmp     .LBB7390_611
.LBB7390_607:
 leaq    .L__unnamed_372(%rip), %rcx
 leaq    .L__unnamed_384(%rip), %r8
 movl    $34, %edx
 callq   std::panicking::begin_panic
 jmp     .LBB7390_9
.LBB7390_608:
 leaq    87624(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_603
.LBB7390_609:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_608
.LBB7390_610:
 movb    $0, 117955(%rsp)
 movaps  85328(%rsp), %xmm0
 movaps  %xmm0, 90416(%rsp)
 movaps  85312(%rsp), %xmm0
 movaps  %xmm0, 90400(%rsp)
 movaps  85248(%rsp), %xmm0
 movaps  85264(%rsp), %xmm1
 movaps  85280(%rsp), %xmm2
 movaps  85296(%rsp), %xmm3
 movaps  %xmm3, 90384(%rsp)
 movaps  %xmm2, 90368(%rsp)
 movaps  %xmm1, 90352(%rsp)
 movaps  %xmm0, 90336(%rsp)
 leaq    90440(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_615
.LBB7390_611:
 leaq    87624(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_610
.LBB7390_612:
 jmp     .LBB7390_614
.LBB7390_613:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_612
.LBB7390_614:
 testb   $1, 117953(%rsp)
 jne     .LBB7390_620
 jmp     .LBB7390_583
.LBB7390_615:
 movb    $0, 117953(%rsp)
 movq    87616(%rsp), %rcx
 movq    %rcx, 90480(%rsp)
 movups  87600(%rsp), %xmm0
 movaps  %xmm0, 90464(%rsp)
 movaps  90416(%rsp), %xmm0
 movups  %xmm0, 90208(%rsp)
 movaps  90400(%rsp), %xmm0
 movups  %xmm0, 90192(%rsp)
 movaps  90336(%rsp), %xmm0
 movaps  90352(%rsp), %xmm1
 movaps  90368(%rsp), %xmm2
 movaps  90384(%rsp), %xmm3
 movups  %xmm3, 90176(%rsp)
 movups  %xmm2, 90160(%rsp)
 movups  %xmm1, 90144(%rsp)
 movups  %xmm0, 90128(%rsp)
 movq    90456(%rsp), %rcx
 movq    %rcx, 90240(%rsp)
 movups  90440(%rsp), %xmm0
 movups  %xmm0, 90224(%rsp)
 movq    90480(%rsp), %rcx
 movq    %rcx, 90264(%rsp)
 movaps  90464(%rsp), %xmm0
 movups  %xmm0, 90248(%rsp)
 movb    $26, 90120(%rsp)
 leaq    72784(%rsp), %rcx
 leaq    90120(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_618
.LBB7390_616:
 leaq    90336(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::intermediate_type::IntermediateType>
 jmp     .LBB7390_614
.LBB7390_617:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_616
.LBB7390_618:
 movb    $0, 117953(%rsp)
 movb    $0, 117955(%rsp)
 jmp     .LBB7390_559
.LBB7390_619:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_614
.LBB7390_620:
 leaq    87600(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_583
.LBB7390_621:
 leaq    85248(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::intermediate_type::IntermediateType>
 jmp     .LBB7390_468
.LBB7390_622:
 leaq    73384(%rsp), %rcx
 leaq    73648(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_623
.LBB7390_623:
 movq    73384(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_625
 jmp     .LBB7390_1723
.LBB7390_1723:
 jmp     .LBB7390_626
 ud2
.LBB7390_625:
 movups  73392(%rsp), %xmm0
 movups  73408(%rsp), %xmm1
 movaps  %xmm1, 74448(%rsp)
 movaps  %xmm0, 74432(%rsp)
 movb    $1, 117962(%rsp)
 movaps  74432(%rsp), %xmm0
 movaps  74448(%rsp), %xmm1
 movaps  %xmm1, 73360(%rsp)
 movaps  %xmm0, 73344(%rsp)
 leaq    74472(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_631
.LBB7390_626:
 leaq    73392(%rsp), %rdx
 leaq    73920(%rsp), %rcx
 movq    %rcx, 1560(%rsp)
 movl    $256, %r8d
 movq    %r8, 1568(%rsp)
 callq   memcpy
 movq    1560(%rsp), %rdx
 movq    1568(%rsp), %r8
 leaq    74176(%rsp), %rcx
 movq    %rcx, 1576(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1576(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_627
.LBB7390_627:
 movb    $0, 117962(%rsp)
 jmp     .LBB7390_630
.LBB7390_628:
 jmp     .LBB7390_468
.LBB7390_629:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_628
.LBB7390_630:
 jmp     .LBB7390_543
.LBB7390_631:
 movq    2792(%rsp), %rdx
 movb    $0, 117962(%rsp)
 leaq    73080(%rsp), %rcx
 leaq    73344(%rsp), %r8
 leaq    74472(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_634
.LBB7390_632:
 testb   $1, 117962(%rsp)
 jne     .LBB7390_635
 jmp     .LBB7390_628
.LBB7390_633:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_632
.LBB7390_634:
 movb    $0, 117962(%rsp)
 leaq    72816(%rsp), %rcx
 leaq    73080(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_636
.LBB7390_635:
 leaq    73344(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_628
.LBB7390_636:
 movq    72816(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_638
 jmp     .LBB7390_1724
.LBB7390_1724:
 jmp     .LBB7390_639
 ud2
.LBB7390_638:
 movq    72824(%rsp), %rax
 movq    %rax, 75008(%rsp)
 movq    72832(%rsp), %rax
 movq    %rax, 75016(%rsp)
 movq    72840(%rsp), %rax
 movq    %rax, 75024(%rsp)
 movq    72848(%rsp), %rax
 movq    %rax, 75032(%rsp)
 movq    75008(%rsp), %rax
 movq    %rax, 72784(%rsp)
 movq    75016(%rsp), %rax
 movq    %rax, 72792(%rsp)
 movq    75024(%rsp), %rax
 movq    %rax, 72800(%rsp)
 movq    75032(%rsp), %rax
 movq    %rax, 72808(%rsp)
 jmp     .LBB7390_559
.LBB7390_639:
 leaq    72824(%rsp), %rdx
 leaq    74496(%rsp), %rcx
 movq    %rcx, 1536(%rsp)
 movl    $256, %r8d
 movq    %r8, 1544(%rsp)
 callq   memcpy
 movq    1536(%rsp), %rdx
 movq    1544(%rsp), %r8
 leaq    74752(%rsp), %rcx
 movq    %rcx, 1552(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1552(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_640
.LBB7390_640:
 jmp     .LBB7390_630
.LBB7390_641:
 jmp     .LBB7390_628
.LBB7390_642:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_641
.LBB7390_643:
 leaq    75608(%rsp), %rcx
 leaq    75872(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_644
.LBB7390_644:
 movq    75608(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_646
 jmp     .LBB7390_1725
.LBB7390_1725:
 jmp     .LBB7390_647
 ud2
.LBB7390_646:
 movups  75616(%rsp), %xmm0
 movups  75632(%rsp), %xmm1
 movaps  %xmm1, 76672(%rsp)
 movaps  %xmm0, 76656(%rsp)
 movb    $1, 117961(%rsp)
 movaps  76656(%rsp), %xmm0
 movaps  76672(%rsp), %xmm1
 movaps  %xmm1, 75584(%rsp)
 movaps  %xmm0, 75568(%rsp)
 leaq    76688(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_652
.LBB7390_647:
 leaq    75616(%rsp), %rdx
 leaq    76144(%rsp), %rcx
 movq    %rcx, 1512(%rsp)
 movl    $256, %r8d
 movq    %r8, 1520(%rsp)
 callq   memcpy
 movq    1512(%rsp), %rdx
 movq    1520(%rsp), %r8
 leaq    76400(%rsp), %rcx
 movq    %rcx, 1528(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1528(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_648
.LBB7390_648:
 movb    $0, 117961(%rsp)
 jmp     .LBB7390_651
.LBB7390_649:
 jmp     .LBB7390_468
.LBB7390_650:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_649
.LBB7390_651:
 jmp     .LBB7390_543
.LBB7390_652:
 movq    2792(%rsp), %rdx
 movb    $0, 117961(%rsp)
 leaq    75304(%rsp), %rcx
 leaq    75568(%rsp), %r8
 leaq    76688(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_655
.LBB7390_653:
 testb   $1, 117961(%rsp)
 jne     .LBB7390_656
 jmp     .LBB7390_649
.LBB7390_654:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_653
.LBB7390_655:
 movb    $0, 117961(%rsp)
 leaq    75040(%rsp), %rcx
 leaq    75304(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_657
.LBB7390_656:
 leaq    75568(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_649
.LBB7390_657:
 movq    75040(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_659
 jmp     .LBB7390_1726
.LBB7390_1726:
 jmp     .LBB7390_660
 ud2
.LBB7390_659:
 movq    75048(%rsp), %rax
 movq    %rax, 77224(%rsp)
 movq    75056(%rsp), %rax
 movq    %rax, 77232(%rsp)
 movq    75064(%rsp), %rax
 movq    %rax, 77240(%rsp)
 movq    75072(%rsp), %rax
 movq    %rax, 77248(%rsp)
 movq    77224(%rsp), %rax
 movq    %rax, 72784(%rsp)
 movq    77232(%rsp), %rax
 movq    %rax, 72792(%rsp)
 movq    77240(%rsp), %rax
 movq    %rax, 72800(%rsp)
 movq    77248(%rsp), %rax
 movq    %rax, 72808(%rsp)
 jmp     .LBB7390_559
.LBB7390_660:
 leaq    75048(%rsp), %rdx
 leaq    76712(%rsp), %rcx
 movq    %rcx, 1488(%rsp)
 movl    $256, %r8d
 movq    %r8, 1496(%rsp)
 callq   memcpy
 movq    1488(%rsp), %rdx
 movq    1496(%rsp), %r8
 leaq    76968(%rsp), %rcx
 movq    %rcx, 1504(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1504(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_661
.LBB7390_661:
 jmp     .LBB7390_651
.LBB7390_662:
 jmp     .LBB7390_649
.LBB7390_663:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_662
.LBB7390_664:
 jmp     .LBB7390_559
.LBB7390_665:
 movq    90560(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_669
 jmp     .LBB7390_1737
.LBB7390_1737:
 jmp     .LBB7390_670
.LBB7390_666:
 leaq    72784(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_468
.LBB7390_667:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_666
 ud2
.LBB7390_669:
 movups  90568(%rsp), %xmm0
 movups  90584(%rsp), %xmm1
 movaps  %xmm1, 90848(%rsp)
 movaps  %xmm0, 90832(%rsp)
 movb    $1, 117952(%rsp)
 movaps  90832(%rsp), %xmm0
 movaps  90848(%rsp), %xmm1
 movaps  %xmm1, 91408(%rsp)
 movaps  %xmm0, 91392(%rsp)
 movl    $32, %ecx
 movl    $8, %edx
 callq   alloc::alloc::exchange_malloc
 movq    %rax, %rdx
 movups  72784(%rsp), %xmm0
 movups  72800(%rsp), %xmm1
 movaps  %xmm1, 91472(%rsp)
 movaps  %xmm0, 91456(%rsp)
 movaps  91456(%rsp), %xmm0
 movaps  91472(%rsp), %xmm1
 movups  %xmm1, 16(%rdx)
 movups  %xmm0, (%rdx)
 leaq    91432(%rsp), %rcx
 movl    $1, %r8d
 callq   alloc::slice::<impl [T]>::into_vec
 jmp     .LBB7390_672
.LBB7390_670:
 movq    72784(%rsp), %rax
 movq    %rax, 3376(%rsp)
 movq    72792(%rsp), %rax
 movq    %rax, 3384(%rsp)
 movq    72800(%rsp), %rax
 movq    %rax, 3392(%rsp)
 movq    72808(%rsp), %rax
 movq    %rax, 3400(%rsp)
.LBB7390_671:
 leaq    72688(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_691
.LBB7390_672:
 movq    2792(%rsp), %rdx
 movb    $0, 117952(%rsp)
 leaq    91128(%rsp), %rcx
 leaq    91392(%rsp), %r8
 leaq    91432(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_675
.LBB7390_673:
 testb   $1, 117952(%rsp)
 jne     .LBB7390_677
 jmp     .LBB7390_676
.LBB7390_674:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_673
.LBB7390_675:
 movb    $0, 117952(%rsp)
 leaq    90864(%rsp), %rcx
 leaq    91128(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_678
.LBB7390_676:
 leaq    72688(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_686
.LBB7390_677:
 leaq    91392(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_676
.LBB7390_678:
 movq    90864(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_681
 jmp     .LBB7390_1738
.LBB7390_1738:
 jmp     .LBB7390_682
.LBB7390_679:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_676
 ud2
.LBB7390_681:
 movq    90872(%rsp), %rax
 movq    %rax, 92000(%rsp)
 movq    90880(%rsp), %rax
 movq    %rax, 92008(%rsp)
 movq    90888(%rsp), %rax
 movq    %rax, 92016(%rsp)
 movq    90896(%rsp), %rax
 movq    %rax, 92024(%rsp)
 movq    92000(%rsp), %rax
 movq    %rax, 3376(%rsp)
 movq    92008(%rsp), %rax
 movq    %rax, 3384(%rsp)
 movq    92016(%rsp), %rax
 movq    %rax, 3392(%rsp)
 movq    92024(%rsp), %rax
 movq    %rax, 3400(%rsp)
 jmp     .LBB7390_671
.LBB7390_682:
 leaq    90872(%rsp), %rdx
 leaq    91488(%rsp), %rcx
 movq    %rcx, 1464(%rsp)
 movl    $256, %r8d
 movq    %r8, 1472(%rsp)
 callq   memcpy
 movq    1464(%rsp), %rdx
 movq    1472(%rsp), %r8
 leaq    91744(%rsp), %rcx
 movq    %rcx, 1480(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1480(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_683
.LBB7390_683:
 leaq    72688(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_687
.LBB7390_684:
 jmp     .LBB7390_676
.LBB7390_685:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_684
.LBB7390_686:
 cmpq    $0, 90560(%rsp)
 je      .LBB7390_85
 jmp     .LBB7390_692
.LBB7390_687:
 cmpq    $0, 90560(%rsp)
 je      .LBB7390_689
 jmp     .LBB7390_690
.LBB7390_688:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_686
.LBB7390_689:
 jmp     .LBB7390_104
.LBB7390_690:
 leaq    90560(%rsp), %rcx
 callq   core::ptr::drop_in_place<core::result::Result<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>,jodin_rs::core::error::JodinError>>
 jmp     .LBB7390_689
.LBB7390_691:
 cmpq    $0, 90560(%rsp)
 je      .LBB7390_693
 jmp     .LBB7390_694
.LBB7390_692:
 leaq    90560(%rsp), %rcx
 callq   core::ptr::drop_in_place<core::result::Result<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>,jodin_rs::core::error::JodinError>>
 jmp     .LBB7390_85
.LBB7390_693:
 jmp     .LBB7390_89
.LBB7390_694:
 leaq    90560(%rsp), %rcx
 callq   core::ptr::drop_in_place<core::result::Result<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>,jodin_rs::core::error::JodinError>>
 jmp     .LBB7390_693
.LBB7390_695:
 movq    1832(%rsp), %rcx
 movq    1840(%rsp), %rdx
 movq    %rdx, 90536(%rsp)
 movq    %rcx, 90544(%rsp)
 movq    %rsp, %rcx
 movq    $1, 32(%rcx)
 leaq    .L__unnamed_385(%rip), %rdx
 leaq    90488(%rsp), %rcx
 movl    $1, %r8d
 leaq    90536(%rsp), %r9
 callq   core::fmt::Arguments::new_v1
 jmp     .LBB7390_696
.LBB7390_696:
 leaq    .L__unnamed_386(%rip), %rdx
 leaq    90488(%rsp), %rcx
 callq   std::panicking::begin_panic_fmt
 jmp     .LBB7390_9
.LBB7390_697:
 leaq    .L__unnamed_387(%rip), %r8
 leaq    110920(%rsp), %rcx
 leaq    110952(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_700
.LBB7390_698:
 leaq    110344(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_85
.LBB7390_699:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_698
.LBB7390_700:
 movb    $1, 117928(%rsp)
 leaq    110984(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_701
.LBB7390_701:
 movq    2792(%rsp), %rdx
 movb    $0, 117928(%rsp)
 leaq    110656(%rsp), %rcx
 leaq    110920(%rsp), %r8
 leaq    110984(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_704
.LBB7390_702:
 testb   $1, 117928(%rsp)
 jne     .LBB7390_705
 jmp     .LBB7390_698
.LBB7390_703:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_702
.LBB7390_704:
 movb    $0, 117928(%rsp)
 leaq    110392(%rsp), %rcx
 leaq    110656(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_706
.LBB7390_705:
 leaq    110920(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_698
.LBB7390_706:
 movq    110392(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_708
 jmp     .LBB7390_1739
.LBB7390_1739:
 jmp     .LBB7390_709
 ud2
.LBB7390_708:
 movups  110400(%rsp), %xmm0
 movups  110416(%rsp), %xmm1
 movaps  %xmm1, 111536(%rsp)
 movaps  %xmm0, 111520(%rsp)
 movb    $1, 117927(%rsp)
 movaps  111520(%rsp), %xmm0
 movaps  111536(%rsp), %xmm1
 movaps  %xmm1, 110368(%rsp)
 movaps  %xmm0, 110352(%rsp)
 leaq    111704(%rsp), %rcx
 leaq    110344(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_714
.LBB7390_709:
 leaq    110400(%rsp), %rdx
 leaq    111008(%rsp), %rcx
 movq    %rcx, 1440(%rsp)
 movl    $256, %r8d
 movq    %r8, 1448(%rsp)
 callq   memcpy
 movq    1440(%rsp), %rdx
 movq    1448(%rsp), %r8
 leaq    111264(%rsp), %rcx
 movq    %rcx, 1456(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1456(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_710
.LBB7390_710:
 jmp     .LBB7390_713
.LBB7390_711:
 jmp     .LBB7390_698
.LBB7390_712:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_711
.LBB7390_713:
 movb    $0, 117927(%rsp)
 leaq    110344(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_104
.LBB7390_714:
 leaq    .L__unnamed_388(%rip), %r8
 leaq    111672(%rsp), %rcx
 leaq    111704(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_717
.LBB7390_715:
 testb   $1, 117927(%rsp)
 jne     .LBB7390_757
 jmp     .LBB7390_698
.LBB7390_716:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_715
.LBB7390_717:
 leaq    111632(%rsp), %rcx
 leaq    111672(%rsp), %rdx
 callq   pest::iterators::pair::Pair<R>::into_inner
 jmp     .LBB7390_718
.LBB7390_718:
 leaq    111600(%rsp), %rcx
 leaq    111632(%rsp), %rdx
 callq   <pest::iterators::pairs::Pairs<R> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_719
.LBB7390_719:
 leaq    .L__unnamed_389(%rip), %r8
 leaq    111568(%rsp), %rcx
 leaq    111600(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_722
.LBB7390_720:
 leaq    111632(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_715
.LBB7390_721:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_720
.LBB7390_722:
 leaq    111568(%rsp), %rcx
 callq   pest::iterators::pair::Pair<R>::as_rule
 movb    %al, %cl
 movb    %cl, 1439(%rsp)
 jmp     .LBB7390_723
.LBB7390_723:
 movb    1439(%rsp), %cl
 movb    %cl, 111567(%rsp)
 leaq    111568(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_726
.LBB7390_724:
 leaq    111568(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_720
.LBB7390_725:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_724
.LBB7390_726:
 leaq    111632(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_727
.LBB7390_727:
 movzbl  111567(%rsp), %eax
 addq    $-76, %rax
 movq    %rax, 1424(%rsp)
 subq    $8, %rax
 ja      .LBB7390_728
 movq    1424(%rsp), %rax
 leaq    .LJTI7390_1(%rip), %rcx
 movslq  (%rcx, %rax, 4), %rax
 addq    %rcx, %rax
 jmpq    *%rax
.LBB7390_728:
 leaq    .L__unnamed_181(%rip), %rcx
 leaq    .L__unnamed_390(%rip), %r8
 movl    $40, %edx
 callq   core::panicking::panic
 jmp     .LBB7390_9
.LBB7390_729:
 movb    $26, 111743(%rsp)
 jmp     .LBB7390_738
.LBB7390_730:
 movb    $3, 111743(%rsp)
 jmp     .LBB7390_738
.LBB7390_731:
 movb    $4, 111743(%rsp)
 jmp     .LBB7390_738
.LBB7390_732:
 movb    $5, 111743(%rsp)
 jmp     .LBB7390_738
.LBB7390_733:
 movb    $7, 111743(%rsp)
 jmp     .LBB7390_738
.LBB7390_734:
 movb    $6, 111743(%rsp)
 jmp     .LBB7390_738
.LBB7390_735:
 movb    $10, 111743(%rsp)
 jmp     .LBB7390_738
.LBB7390_736:
 movb    $9, 111743(%rsp)
 jmp     .LBB7390_738
.LBB7390_737:
 movb    $8, 111743(%rsp)
.LBB7390_738:
 leaq    112344(%rsp), %rcx
 leaq    110344(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_739
.LBB7390_739:
 leaq    .L__unnamed_391(%rip), %r8
 leaq    112312(%rsp), %rcx
 leaq    112344(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_740
.LBB7390_740:
 movb    $1, 117926(%rsp)
 leaq    112376(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_741
.LBB7390_741:
 movq    2792(%rsp), %rdx
 movb    $0, 117926(%rsp)
 leaq    112048(%rsp), %rcx
 leaq    112312(%rsp), %r8
 leaq    112376(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_744
.LBB7390_742:
 testb   $1, 117926(%rsp)
 jne     .LBB7390_745
 jmp     .LBB7390_715
.LBB7390_743:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_742
.LBB7390_744:
 movb    $0, 117926(%rsp)
 leaq    111784(%rsp), %rcx
 leaq    112048(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_746
.LBB7390_745:
 leaq    112312(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_715
.LBB7390_746:
 movq    111784(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_748
 jmp     .LBB7390_1741
.LBB7390_1741:
 jmp     .LBB7390_749
 ud2
.LBB7390_748:
 movups  111792(%rsp), %xmm0
 movups  111808(%rsp), %xmm1
 movaps  %xmm1, 112928(%rsp)
 movaps  %xmm0, 112912(%rsp)
 movb    $1, 117925(%rsp)
 movaps  112912(%rsp), %xmm0
 movaps  112928(%rsp), %xmm1
 movaps  %xmm1, 111760(%rsp)
 movaps  %xmm0, 111744(%rsp)
 movb    111743(%rsp), %cl
 movb    $0, 117927(%rsp)
 movaps  110352(%rsp), %xmm0
 movaps  110368(%rsp), %xmm1
 movaps  %xmm1, 113184(%rsp)
 movaps  %xmm0, 113168(%rsp)
 movb    $0, 117925(%rsp)
 movaps  111744(%rsp), %xmm0
 movaps  111760(%rsp), %xmm1
 movaps  %xmm1, 113216(%rsp)
 movaps  %xmm0, 113200(%rsp)
 movb    %cl, 112953(%rsp)
 movaps  113168(%rsp), %xmm0
 movaps  113184(%rsp), %xmm1
 movups  %xmm1, 112976(%rsp)
 movups  %xmm0, 112960(%rsp)
 movaps  113200(%rsp), %xmm0
 movaps  113216(%rsp), %xmm1
 movups  %xmm1, 113008(%rsp)
 movups  %xmm0, 112992(%rsp)
 movb    $36, 112952(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    112952(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_753
.LBB7390_749:
 leaq    111792(%rsp), %rdx
 leaq    112400(%rsp), %rcx
 movq    %rcx, 1400(%rsp)
 movl    $256, %r8d
 movq    %r8, 1408(%rsp)
 callq   memcpy
 movq    1400(%rsp), %rdx
 movq    1408(%rsp), %r8
 leaq    112656(%rsp), %rcx
 movq    %rcx, 1416(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1416(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_750
.LBB7390_750:
 movb    $0, 117925(%rsp)
 leaq    110352(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_713
.LBB7390_751:
 jmp     .LBB7390_715
.LBB7390_752:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_751
.LBB7390_753:
 movb    $0, 117925(%rsp)
 movb    $0, 117927(%rsp)
 leaq    110344(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_89
.LBB7390_754:
 testb   $1, 117925(%rsp)
 jne     .LBB7390_756
 jmp     .LBB7390_715
.LBB7390_755:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_754
.LBB7390_756:
 leaq    111744(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_715
.LBB7390_757:
 leaq    110352(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_698
.LBB7390_758:
 movb    $-86, 16463(%rsp)
 movb    16463(%rsp), %r8b
 leaq    16192(%rsp), %rcx
 leaq    15168(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_759
.LBB7390_759:
 leaq    15928(%rsp), %rcx
 leaq    16192(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_762
.LBB7390_760:
 leaq    15168(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_85
.LBB7390_761:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_760
.LBB7390_762:
 movq    15928(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_764
 jmp     .LBB7390_1742
.LBB7390_1742:
 jmp     .LBB7390_765
 ud2
.LBB7390_764:
 movq    2792(%rsp), %rdx
 movups  15936(%rsp), %xmm0
 movups  15952(%rsp), %xmm1
 movaps  %xmm1, 16992(%rsp)
 movaps  %xmm0, 16976(%rsp)
 movaps  16976(%rsp), %xmm0
 movaps  16992(%rsp), %xmm1
 movaps  %xmm1, 15904(%rsp)
 movaps  %xmm0, 15888(%rsp)
 leaq    15624(%rsp), %rcx
 leaq    15888(%rsp), %r8
 callq   jodin_rs::ast::JodinNodeGenerator::new_intermediate_type
 jmp     .LBB7390_770
.LBB7390_765:
 leaq    15936(%rsp), %rdx
 leaq    16464(%rsp), %rcx
 movq    %rcx, 1376(%rsp)
 movl    $256, %r8d
 movq    %r8, 1384(%rsp)
 callq   memcpy
 movq    1376(%rsp), %rdx
 movq    1384(%rsp), %r8
 leaq    16720(%rsp), %rcx
 movq    %rcx, 1392(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1392(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_766
.LBB7390_766:
 jmp     .LBB7390_769
.LBB7390_767:
 jmp     .LBB7390_760
.LBB7390_768:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_767
.LBB7390_769:
 jmp     .LBB7390_778
.LBB7390_770:
 leaq    15360(%rsp), %rcx
 leaq    15624(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_771
.LBB7390_771:
 movq    15360(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_773
 jmp     .LBB7390_1743
.LBB7390_1743:
 jmp     .LBB7390_774
 ud2
.LBB7390_773:
 movups  15448(%rsp), %xmm0
 movaps  %xmm0, 17600(%rsp)
 movups  15432(%rsp), %xmm0
 movaps  %xmm0, 17584(%rsp)
 movups  15368(%rsp), %xmm0
 movups  15384(%rsp), %xmm1
 movups  15400(%rsp), %xmm2
 movups  15416(%rsp), %xmm3
 movaps  %xmm3, 17568(%rsp)
 movaps  %xmm2, 17552(%rsp)
 movaps  %xmm1, 17536(%rsp)
 movaps  %xmm0, 17520(%rsp)
 movb    $1, 118005(%rsp)
 movaps  17600(%rsp), %xmm0
 movaps  %xmm0, 15344(%rsp)
 movaps  17584(%rsp), %xmm0
 movaps  %xmm0, 15328(%rsp)
 movaps  17520(%rsp), %xmm0
 movaps  17536(%rsp), %xmm1
 movaps  17552(%rsp), %xmm2
 movaps  17568(%rsp), %xmm3
 movaps  %xmm3, 15312(%rsp)
 movaps  %xmm2, 15296(%rsp)
 movaps  %xmm1, 15280(%rsp)
 movaps  %xmm0, 15264(%rsp)
 movb    $-108, 18751(%rsp)
 movb    18751(%rsp), %r8b
 leaq    18480(%rsp), %rcx
 leaq    15168(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_779
.LBB7390_774:
 leaq    15368(%rsp), %rdx
 leaq    17008(%rsp), %rcx
 movq    %rcx, 1352(%rsp)
 movl    $256, %r8d
 movq    %r8, 1360(%rsp)
 callq   memcpy
 movq    1352(%rsp), %rdx
 movq    1360(%rsp), %r8
 leaq    17264(%rsp), %rcx
 movq    %rcx, 1368(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1368(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_775
.LBB7390_775:
 jmp     .LBB7390_769
.LBB7390_776:
 jmp     .LBB7390_767
.LBB7390_777:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_776
.LBB7390_778:
 movb    $0, 118005(%rsp)
 leaq    15168(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_104
.LBB7390_779:
 leaq    18216(%rsp), %rcx
 leaq    18480(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_782
.LBB7390_780:
 testb   $1, 118005(%rsp)
 jne     .LBB7390_806
 jmp     .LBB7390_760
.LBB7390_781:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_780
.LBB7390_782:
 movq    18216(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_784
 jmp     .LBB7390_1744
.LBB7390_1744:
 jmp     .LBB7390_785
 ud2
.LBB7390_784:
 movups  18224(%rsp), %xmm0
 movups  18240(%rsp), %xmm1
 movaps  %xmm1, 19280(%rsp)
 movaps  %xmm0, 19264(%rsp)
 movb    $1, 118004(%rsp)
 movaps  19264(%rsp), %xmm0
 movaps  19280(%rsp), %xmm1
 movaps  %xmm1, 18192(%rsp)
 movaps  %xmm0, 18176(%rsp)
 leaq    19304(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_790
.LBB7390_785:
 leaq    18224(%rsp), %rdx
 leaq    18752(%rsp), %rcx
 movq    %rcx, 1328(%rsp)
 movl    $256, %r8d
 movq    %r8, 1336(%rsp)
 callq   memcpy
 movq    1328(%rsp), %rdx
 movq    1336(%rsp), %r8
 leaq    19008(%rsp), %rcx
 movq    %rcx, 1344(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1344(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_786
.LBB7390_786:
 movb    $0, 118004(%rsp)
 jmp     .LBB7390_789
.LBB7390_787:
 jmp     .LBB7390_780
.LBB7390_788:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_787
.LBB7390_789:
 movb    $0, 118003(%rsp)
 leaq    15264(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::intermediate_type::IntermediateType>
 jmp     .LBB7390_778
.LBB7390_790:
 movq    2792(%rsp), %rdx
 movb    $0, 118004(%rsp)
 leaq    17912(%rsp), %rcx
 leaq    18176(%rsp), %r8
 leaq    19304(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_793
.LBB7390_791:
 testb   $1, 118004(%rsp)
 jne     .LBB7390_794
 jmp     .LBB7390_787
.LBB7390_792:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_791
.LBB7390_793:
 movb    $0, 118004(%rsp)
 leaq    17648(%rsp), %rcx
 leaq    17912(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_795
.LBB7390_794:
 leaq    18176(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_787
.LBB7390_795:
 movq    17648(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_797
 jmp     .LBB7390_1745
.LBB7390_1745:
 jmp     .LBB7390_798
 ud2
.LBB7390_797:
 movups  17656(%rsp), %xmm0
 movups  17672(%rsp), %xmm1
 movaps  %xmm1, 19856(%rsp)
 movaps  %xmm0, 19840(%rsp)
 movb    $1, 118003(%rsp)
 movaps  19840(%rsp), %xmm0
 movaps  19856(%rsp), %xmm1
 movaps  %xmm1, 17632(%rsp)
 movaps  %xmm0, 17616(%rsp)
 movb    $0, 118005(%rsp)
 movaps  15344(%rsp), %xmm0
 movaps  %xmm0, 20176(%rsp)
 movaps  15328(%rsp), %xmm0
 movaps  %xmm0, 20160(%rsp)
 movaps  15264(%rsp), %xmm0
 movaps  15280(%rsp), %xmm1
 movaps  15296(%rsp), %xmm2
 movaps  15312(%rsp), %xmm3
 movaps  %xmm3, 20144(%rsp)
 movaps  %xmm2, 20128(%rsp)
 movaps  %xmm1, 20112(%rsp)
 movaps  %xmm0, 20096(%rsp)
 movb    $0, 118003(%rsp)
 movaps  17616(%rsp), %xmm0
 movaps  17632(%rsp), %xmm1
 movaps  %xmm1, 20208(%rsp)
 movaps  %xmm0, 20192(%rsp)
 movaps  20176(%rsp), %xmm0
 movups  %xmm0, 19968(%rsp)
 movaps  20160(%rsp), %xmm0
 movups  %xmm0, 19952(%rsp)
 movaps  20096(%rsp), %xmm0
 movaps  20112(%rsp), %xmm1
 movaps  20128(%rsp), %xmm2
 movaps  20144(%rsp), %xmm3
 movups  %xmm3, 19936(%rsp)
 movups  %xmm2, 19920(%rsp)
 movups  %xmm1, 19904(%rsp)
 movups  %xmm0, 19888(%rsp)
 movaps  20192(%rsp), %xmm0
 movaps  20208(%rsp), %xmm1
 movups  %xmm1, 20000(%rsp)
 movups  %xmm0, 19984(%rsp)
 movb    $10, 19880(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    19880(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_802
.LBB7390_798:
 leaq    17656(%rsp), %rdx
 leaq    19328(%rsp), %rcx
 movq    %rcx, 1304(%rsp)
 movl    $256, %r8d
 movq    %r8, 1312(%rsp)
 callq   memcpy
 movq    1304(%rsp), %rdx
 movq    1312(%rsp), %r8
 leaq    19584(%rsp), %rcx
 movq    %rcx, 1320(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1320(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_799
.LBB7390_799:
 jmp     .LBB7390_789
.LBB7390_800:
 jmp     .LBB7390_787
.LBB7390_801:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_800
.LBB7390_802:
 movb    $0, 118003(%rsp)
 movb    $0, 118005(%rsp)
 leaq    15168(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_89
.LBB7390_803:
 testb   $1, 118003(%rsp)
 jne     .LBB7390_805
 jmp     .LBB7390_780
.LBB7390_804:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_803
.LBB7390_805:
 leaq    17616(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_780
.LBB7390_806:
 leaq    15264(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::intermediate_type::IntermediateType>
 jmp     .LBB7390_760
.LBB7390_807:
 leaq    14096(%rsp), %rcx
 leaq    14360(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_808
.LBB7390_808:
 movq    14096(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_810
 jmp     .LBB7390_1746
.LBB7390_1746:
 jmp     .LBB7390_811
 ud2
.LBB7390_810:
 movq    14104(%rsp), %rax
 movq    %rax, 15136(%rsp)
 movq    14112(%rsp), %rax
 movq    %rax, 15144(%rsp)
 movq    14120(%rsp), %rax
 movq    %rax, 15152(%rsp)
 movq    14128(%rsp), %rax
 movq    %rax, 15160(%rsp)
 movq    15136(%rsp), %rax
 movq    %rax, 3376(%rsp)
 movq    15144(%rsp), %rax
 movq    %rax, 3384(%rsp)
 movq    15152(%rsp), %rax
 movq    %rax, 3392(%rsp)
 movq    15160(%rsp), %rax
 movq    %rax, 3400(%rsp)
 jmp     .LBB7390_89
.LBB7390_811:
 leaq    14104(%rsp), %rdx
 leaq    14624(%rsp), %rcx
 movq    %rcx, 1280(%rsp)
 movl    $256, %r8d
 movq    %r8, 1288(%rsp)
 callq   memcpy
 movq    1280(%rsp), %rdx
 movq    1288(%rsp), %r8
 leaq    14880(%rsp), %rcx
 movq    %rcx, 1296(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1296(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_812
.LBB7390_812:
 jmp     .LBB7390_104
.LBB7390_813:
 jmp     .LBB7390_85
.LBB7390_814:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_813
.LBB7390_815:
 leaq    13024(%rsp), %rcx
 leaq    13288(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_816
.LBB7390_816:
 movq    13024(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_818
 jmp     .LBB7390_1747
.LBB7390_1747:
 jmp     .LBB7390_819
 ud2
.LBB7390_818:
 movq    13032(%rsp), %rax
 movq    %rax, 14064(%rsp)
 movq    13040(%rsp), %rax
 movq    %rax, 14072(%rsp)
 movq    13048(%rsp), %rax
 movq    %rax, 14080(%rsp)
 movq    13056(%rsp), %rax
 movq    %rax, 14088(%rsp)
 movq    14064(%rsp), %rax
 movq    %rax, 3376(%rsp)
 movq    14072(%rsp), %rax
 movq    %rax, 3384(%rsp)
 movq    14080(%rsp), %rax
 movq    %rax, 3392(%rsp)
 movq    14088(%rsp), %rax
 movq    %rax, 3400(%rsp)
 jmp     .LBB7390_89
.LBB7390_819:
 leaq    13032(%rsp), %rdx
 leaq    13552(%rsp), %rcx
 movq    %rcx, 1256(%rsp)
 movl    $256, %r8d
 movq    %r8, 1264(%rsp)
 callq   memcpy
 movq    1256(%rsp), %rdx
 movq    1264(%rsp), %r8
 leaq    13808(%rsp), %rcx
 movq    %rcx, 1272(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1272(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_820
.LBB7390_820:
 jmp     .LBB7390_104
.LBB7390_821:
 jmp     .LBB7390_85
.LBB7390_822:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_821
.LBB7390_823:
 leaq    11952(%rsp), %rcx
 leaq    12216(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_824
.LBB7390_824:
 movq    11952(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_826
 jmp     .LBB7390_1748
.LBB7390_1748:
 jmp     .LBB7390_827
 ud2
.LBB7390_826:
 movq    11960(%rsp), %rax
 movq    %rax, 12992(%rsp)
 movq    11968(%rsp), %rax
 movq    %rax, 13000(%rsp)
 movq    11976(%rsp), %rax
 movq    %rax, 13008(%rsp)
 movq    11984(%rsp), %rax
 movq    %rax, 13016(%rsp)
 movq    12992(%rsp), %rax
 movq    %rax, 3376(%rsp)
 movq    13000(%rsp), %rax
 movq    %rax, 3384(%rsp)
 movq    13008(%rsp), %rax
 movq    %rax, 3392(%rsp)
 movq    13016(%rsp), %rax
 movq    %rax, 3400(%rsp)
 jmp     .LBB7390_89
.LBB7390_827:
 leaq    11960(%rsp), %rdx
 leaq    12480(%rsp), %rcx
 movq    %rcx, 1232(%rsp)
 movl    $256, %r8d
 movq    %r8, 1240(%rsp)
 callq   memcpy
 movq    1232(%rsp), %rdx
 movq    1240(%rsp), %r8
 leaq    12736(%rsp), %rcx
 movq    %rcx, 1248(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1248(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_828
.LBB7390_828:
 jmp     .LBB7390_104
.LBB7390_829:
 jmp     .LBB7390_85
.LBB7390_830:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_829
.LBB7390_831:
 cmpq    $3, 2848(%rsp)
 je      .LBB7390_843
 jmp     .LBB7390_833
.LBB7390_832:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 movq    %rax, 1224(%rsp)
 subq    $3, %rax
 je      .LBB7390_834
 jmp     .LBB7390_1750
.LBB7390_1750:
 movq    1224(%rsp), %rax
 subq    $7, %rax
 je      .LBB7390_835
 jmp     .LBB7390_1751
.LBB7390_1751:
 movq    1224(%rsp), %rax
 subq    $17, %rax
 je      .LBB7390_836
 jmp     .LBB7390_833
.LBB7390_833:
 leaq    2840(%rsp), %rax
 movq    %rax, 45680(%rsp)
 movq    45680(%rsp), %rcx
 movq    %rcx, 118168(%rsp)
 leaq    _ZN67_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17he8271684dacd979dE(%rip), %rdx
 callq   core::fmt::ArgumentV1::new
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 1208(%rsp)
 movq    %rcx, 1216(%rsp)
 jmp     .LBB7390_867
.LBB7390_834:
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $87, %rax
 je      .LBB7390_841
 jmp     .LBB7390_833
.LBB7390_835:
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $87, %rax
 je      .LBB7390_839
 jmp     .LBB7390_833
.LBB7390_836:
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $87, %rax
 jne     .LBB7390_833
 movq    $0, 45584(%rsp)
 movups  45584(%rsp), %xmm0
 movups  45600(%rsp), %xmm1
 movups  %xmm1, 45392(%rsp)
 movups  %xmm0, 45376(%rsp)
 movb    $21, 45368(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    45368(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_838
.LBB7390_838:
 jmp     .LBB7390_89
.LBB7390_839:
 movb    $22, 43400(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    43400(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_840
.LBB7390_840:
 jmp     .LBB7390_89
.LBB7390_841:
 movb    $23, 43616(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    43616(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_842
.LBB7390_842:
 jmp     .LBB7390_89
.LBB7390_843:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 cmpq    $17, %rax
 jne     .LBB7390_833
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $137, %rax
 jne     .LBB7390_833
 movq    2840(%rsp), %rax
 movzbl  2(%rax), %eax
 cmpq    $87, %rax
 jne     .LBB7390_833
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %rcx
 movq    %rcx, 43832(%rsp)
 leaq    43872(%rsp), %rcx
 leaq    43832(%rsp), %rdx
 movl    $1, %r8d
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_847
.LBB7390_847:
 leaq    .L__unnamed_392(%rip), %r8
 leaq    43840(%rsp), %rcx
 leaq    43872(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_850
.LBB7390_848:
 leaq    43832(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_85
.LBB7390_849:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_848
.LBB7390_850:
 movb    $1, 117982(%rsp)
 movups  43840(%rsp), %xmm0
 movups  43856(%rsp), %xmm1
 movaps  %xmm1, 44480(%rsp)
 movaps  %xmm0, 44464(%rsp)
 leaq    44504(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_851
.LBB7390_851:
 movq    2792(%rsp), %rdx
 movb    $0, 117982(%rsp)
 leaq    44200(%rsp), %rcx
 leaq    44464(%rsp), %r8
 leaq    44504(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_854
.LBB7390_852:
 testb   $1, 117982(%rsp)
 jne     .LBB7390_855
 jmp     .LBB7390_848
.LBB7390_853:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_852
.LBB7390_854:
 movb    $0, 117982(%rsp)
 leaq    43936(%rsp), %rcx
 leaq    44200(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_856
.LBB7390_855:
 leaq    44464(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_848
.LBB7390_856:
 movq    43936(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_858
 jmp     .LBB7390_1749
.LBB7390_1749:
 jmp     .LBB7390_859
 ud2
.LBB7390_858:
 movups  43944(%rsp), %xmm0
 movups  43960(%rsp), %xmm1
 movaps  %xmm1, 45056(%rsp)
 movaps  %xmm0, 45040(%rsp)
 movb    $1, 117981(%rsp)
 movaps  45040(%rsp), %xmm0
 movaps  45056(%rsp), %xmm1
 movaps  %xmm1, 43920(%rsp)
 movaps  %xmm0, 43904(%rsp)
 movb    $0, 117981(%rsp)
 movaps  43904(%rsp), %xmm0
 movaps  43920(%rsp), %xmm1
 movaps  %xmm1, 45344(%rsp)
 movaps  %xmm0, 45328(%rsp)
 movaps  45328(%rsp), %xmm0
 movaps  45344(%rsp), %xmm1
 movaps  %xmm1, 45312(%rsp)
 movaps  %xmm0, 45296(%rsp)
 movaps  45296(%rsp), %xmm0
 movaps  45312(%rsp), %xmm1
 movups  %xmm1, 45104(%rsp)
 movups  %xmm0, 45088(%rsp)
 movb    $21, 45080(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    45080(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_863
.LBB7390_859:
 leaq    43944(%rsp), %rdx
 leaq    44528(%rsp), %rcx
 movq    %rcx, 1184(%rsp)
 movl    $256, %r8d
 movq    %r8, 1192(%rsp)
 callq   memcpy
 movq    1184(%rsp), %rdx
 movq    1192(%rsp), %r8
 leaq    44784(%rsp), %rcx
 movq    %rcx, 1200(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1200(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_860
.LBB7390_860:
 movb    $0, 117981(%rsp)
 leaq    43832(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_104
.LBB7390_861:
 jmp     .LBB7390_848
.LBB7390_862:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_861
.LBB7390_863:
 movb    $0, 117981(%rsp)
 leaq    43832(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_89
.LBB7390_864:
 testb   $1, 117981(%rsp)
 jne     .LBB7390_866
 jmp     .LBB7390_848
.LBB7390_865:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_864
.LBB7390_866:
 leaq    43904(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_848
.LBB7390_867:
 movq    1208(%rsp), %rcx
 movq    1216(%rsp), %rdx
 movq    %rdx, 45664(%rsp)
 movq    %rcx, 45672(%rsp)
 movq    %rsp, %rcx
 movq    $1, 32(%rcx)
 leaq    .L__unnamed_393(%rip), %rdx
 leaq    45616(%rsp), %rcx
 movl    $1, %r8d
 leaq    45664(%rsp), %r9
 callq   core::fmt::Arguments::new_v1
 jmp     .LBB7390_868
.LBB7390_868:
 leaq    .L__unnamed_394(%rip), %rdx
 leaq    45616(%rsp), %rcx
 callq   std::panicking::begin_panic_fmt
 jmp     .LBB7390_9
.LBB7390_869:
 cmpq    $4, 2848(%rsp)
 je      .LBB7390_908
 jmp     .LBB7390_907
.LBB7390_870:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 cmpq    $25, %rax
 jne     .LBB7390_872
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $137, %rax
 je      .LBB7390_873
.LBB7390_872:
 leaq    2840(%rsp), %rax
 movq    %rax, 72680(%rsp)
 movq    72680(%rsp), %rcx
 movq    %rcx, 118176(%rsp)
 leaq    _ZN67_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17he8271684dacd979dE(%rip), %rdx
 callq   core::fmt::ArgumentV1::new
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 1168(%rsp)
 movq    %rcx, 1176(%rsp)
 jmp     .LBB7390_1049
.LBB7390_873:
 movq    2840(%rsp), %rax
 movzbl  2(%rax), %eax
 cmpq    $131, %rax
 jne     .LBB7390_872
 leaq    58912(%rsp), %rcx
 leaq    58312(%rsp), %rdx
 movl    $1, %r8d
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_875
.LBB7390_875:
 leaq    .L__unnamed_395(%rip), %r8
 leaq    58880(%rsp), %rcx
 leaq    58912(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_878
.LBB7390_876:
 leaq    58312(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_85
.LBB7390_877:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_876
.LBB7390_878:
 movq    2792(%rsp), %r8
 leaq    58616(%rsp), %rcx
 leaq    58880(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_879
.LBB7390_879:
 leaq    58352(%rsp), %rcx
 leaq    58616(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_880
.LBB7390_880:
 movq    58352(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_882
 jmp     .LBB7390_1762
.LBB7390_1762:
 jmp     .LBB7390_883
 ud2
.LBB7390_882:
 movups  58360(%rsp), %xmm0
 movups  58376(%rsp), %xmm1
 movaps  %xmm1, 59472(%rsp)
 movaps  %xmm0, 59456(%rsp)
 movb    $1, 117973(%rsp)
 movaps  59456(%rsp), %xmm0
 movaps  59472(%rsp), %xmm1
 movaps  %xmm1, 58336(%rsp)
 movaps  %xmm0, 58320(%rsp)
 xorl    %ecx, %ecx
 movl    %ecx, %r8d
 leaq    60080(%rsp), %rcx
 leaq    58312(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_888
.LBB7390_883:
 leaq    58360(%rsp), %rdx
 leaq    58944(%rsp), %rcx
 movq    %rcx, 1144(%rsp)
 movl    $256, %r8d
 movq    %r8, 1152(%rsp)
 callq   memcpy
 movq    1144(%rsp), %rdx
 movq    1152(%rsp), %r8
 leaq    59200(%rsp), %rcx
 movq    %rcx, 1160(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1160(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_884
.LBB7390_884:
 jmp     .LBB7390_887
.LBB7390_885:
 jmp     .LBB7390_876
.LBB7390_886:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_885
.LBB7390_887:
 movb    $0, 117973(%rsp)
 jmp     .LBB7390_900
.LBB7390_888:
 leaq    .L__unnamed_396(%rip), %r8
 leaq    60048(%rsp), %rcx
 leaq    60080(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_891
.LBB7390_889:
 testb   $1, 117973(%rsp)
 jne     .LBB7390_905
 jmp     .LBB7390_876
.LBB7390_890:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_889
.LBB7390_891:
 movq    2792(%rsp), %r8
 leaq    59784(%rsp), %rcx
 leaq    60048(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_892
.LBB7390_892:
 leaq    59520(%rsp), %rcx
 leaq    59784(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_893
.LBB7390_893:
 movq    59520(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_895
 jmp     .LBB7390_1763
.LBB7390_1763:
 jmp     .LBB7390_896
 ud2
.LBB7390_895:
 movups  59528(%rsp), %xmm0
 movups  59544(%rsp), %xmm1
 movaps  %xmm1, 60640(%rsp)
 movaps  %xmm0, 60624(%rsp)
 movb    $1, 117972(%rsp)
 movaps  60624(%rsp), %xmm0
 movaps  60640(%rsp), %xmm1
 movaps  %xmm1, 59504(%rsp)
 movaps  %xmm0, 59488(%rsp)
 movb    $0, 117973(%rsp)
 movaps  58320(%rsp), %xmm0
 movaps  58336(%rsp), %xmm1
 movaps  %xmm1, 60896(%rsp)
 movaps  %xmm0, 60880(%rsp)
 movb    $0, 117972(%rsp)
 movaps  59488(%rsp), %xmm0
 movaps  59504(%rsp), %xmm1
 movaps  %xmm1, 60928(%rsp)
 movaps  %xmm0, 60912(%rsp)
 movaps  60880(%rsp), %xmm0
 movaps  60896(%rsp), %xmm1
 movups  %xmm1, 60688(%rsp)
 movups  %xmm0, 60672(%rsp)
 movaps  60912(%rsp), %xmm0
 movaps  60928(%rsp), %xmm1
 movups  %xmm1, 60720(%rsp)
 movups  %xmm0, 60704(%rsp)
 movb    $32, 60664(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    60664(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_901
.LBB7390_896:
 leaq    59528(%rsp), %rdx
 leaq    60112(%rsp), %rcx
 movq    %rcx, 1120(%rsp)
 movl    $256, %r8d
 movq    %r8, 1128(%rsp)
 callq   memcpy
 movq    1120(%rsp), %rdx
 movq    1128(%rsp), %r8
 leaq    60368(%rsp), %rcx
 movq    %rcx, 1136(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1136(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_897
.LBB7390_897:
 movb    $0, 117972(%rsp)
 leaq    58320(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_887
.LBB7390_898:
 jmp     .LBB7390_889
.LBB7390_899:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_898
.LBB7390_900:
 leaq    58312(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_104
.LBB7390_901:
 movb    $0, 117972(%rsp)
 movb    $0, 117973(%rsp)
 jmp     .LBB7390_906
.LBB7390_902:
 testb   $1, 117972(%rsp)
 jne     .LBB7390_904
 jmp     .LBB7390_889
.LBB7390_903:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_902
.LBB7390_904:
 leaq    59488(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_889
.LBB7390_905:
 leaq    58320(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_876
.LBB7390_906:
 leaq    58312(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_89
.LBB7390_907:
 cmpq    $5, 2848(%rsp)
 je      .LBB7390_987
 jmp     .LBB7390_872
.LBB7390_908:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 movq    %rax, 1112(%rsp)
 subq    $10, %rax
 je      .LBB7390_909
 jmp     .LBB7390_1756
.LBB7390_1756:
 movq    1112(%rsp), %rax
 subq    $13, %rax
 je      .LBB7390_910
 jmp     .LBB7390_872
.LBB7390_909:
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $131, %rax
 je      .LBB7390_956
 jmp     .LBB7390_872
.LBB7390_910:
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $133, %rax
 jne     .LBB7390_872
 movq    2840(%rsp), %rax
 movzbl  2(%rax), %eax
 cmpq    $133, %rax
 jne     .LBB7390_872
 movq    2840(%rsp), %rax
 movzbl  3(%rax), %eax
 cmpq    $131, %rax
 jne     .LBB7390_872
 leaq    69280(%rsp), %rcx
 leaq    58312(%rsp), %rdx
 movl    $1, %r8d
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_914
.LBB7390_914:
 leaq    .L__unnamed_397(%rip), %r8
 leaq    69248(%rsp), %rcx
 leaq    69280(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_915
.LBB7390_915:
 movq    2792(%rsp), %r8
 leaq    68984(%rsp), %rcx
 leaq    69248(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_916
.LBB7390_916:
 leaq    68720(%rsp), %rcx
 leaq    68984(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_917
.LBB7390_917:
 movq    68720(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_919
 jmp     .LBB7390_1757
.LBB7390_1757:
 jmp     .LBB7390_920
 ud2
.LBB7390_919:
 movups  68728(%rsp), %xmm0
 movups  68744(%rsp), %xmm1
 movaps  %xmm1, 69840(%rsp)
 movaps  %xmm0, 69824(%rsp)
 movb    $1, 117965(%rsp)
 movaps  69824(%rsp), %xmm0
 movaps  69840(%rsp), %xmm1
 movaps  %xmm1, 68704(%rsp)
 movaps  %xmm0, 68688(%rsp)
 xorl    %ecx, %ecx
 movl    %ecx, %r8d
 leaq    70448(%rsp), %rcx
 leaq    58312(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_925
.LBB7390_920:
 leaq    68728(%rsp), %rdx
 leaq    69312(%rsp), %rcx
 movq    %rcx, 1088(%rsp)
 movl    $256, %r8d
 movq    %r8, 1096(%rsp)
 callq   memcpy
 movq    1088(%rsp), %rdx
 movq    1096(%rsp), %r8
 leaq    69568(%rsp), %rcx
 movq    %rcx, 1104(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1104(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_921
.LBB7390_921:
 jmp     .LBB7390_924
.LBB7390_922:
 jmp     .LBB7390_876
.LBB7390_923:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_922
.LBB7390_924:
 movb    $0, 117965(%rsp)
 jmp     .LBB7390_900
.LBB7390_925:
 leaq    .L__unnamed_398(%rip), %r8
 leaq    70416(%rsp), %rcx
 leaq    70448(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_928
.LBB7390_926:
 testb   $1, 117965(%rsp)
 jne     .LBB7390_955
 jmp     .LBB7390_876
.LBB7390_927:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_926
.LBB7390_928:
 movq    2792(%rsp), %r8
 leaq    70152(%rsp), %rcx
 leaq    70416(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_929
.LBB7390_929:
 leaq    69888(%rsp), %rcx
 leaq    70152(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_930
.LBB7390_930:
 movq    69888(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_932
 jmp     .LBB7390_1758
.LBB7390_1758:
 jmp     .LBB7390_933
 ud2
.LBB7390_932:
 movups  69896(%rsp), %xmm0
 movups  69912(%rsp), %xmm1
 movaps  %xmm1, 71008(%rsp)
 movaps  %xmm0, 70992(%rsp)
 movb    $1, 117964(%rsp)
 movaps  70992(%rsp), %xmm0
 movaps  71008(%rsp), %xmm1
 movaps  %xmm1, 69872(%rsp)
 movaps  %xmm0, 69856(%rsp)
 xorl    %ecx, %ecx
 movl    %ecx, %r8d
 leaq    71616(%rsp), %rcx
 leaq    58312(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_938
.LBB7390_933:
 leaq    69896(%rsp), %rdx
 leaq    70480(%rsp), %rcx
 movq    %rcx, 1064(%rsp)
 movl    $256, %r8d
 movq    %r8, 1072(%rsp)
 callq   memcpy
 movq    1064(%rsp), %rdx
 movq    1072(%rsp), %r8
 leaq    70736(%rsp), %rcx
 movq    %rcx, 1080(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1080(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_934
.LBB7390_934:
 jmp     .LBB7390_937
.LBB7390_935:
 jmp     .LBB7390_926
.LBB7390_936:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_935
.LBB7390_937:
 movb    $0, 117964(%rsp)
 leaq    68688(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_924
.LBB7390_938:
 leaq    .L__unnamed_399(%rip), %r8
 leaq    71584(%rsp), %rcx
 leaq    71616(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_941
.LBB7390_939:
 testb   $1, 117964(%rsp)
 jne     .LBB7390_954
 jmp     .LBB7390_926
.LBB7390_940:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_939
.LBB7390_941:
 movq    2792(%rsp), %r8
 leaq    71320(%rsp), %rcx
 leaq    71584(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_942
.LBB7390_942:
 leaq    71056(%rsp), %rcx
 leaq    71320(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_943
.LBB7390_943:
 movq    71056(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_945
 jmp     .LBB7390_1759
.LBB7390_1759:
 jmp     .LBB7390_946
 ud2
.LBB7390_945:
 movups  71064(%rsp), %xmm0
 movups  71080(%rsp), %xmm1
 movaps  %xmm1, 72176(%rsp)
 movaps  %xmm0, 72160(%rsp)
 movb    $1, 117963(%rsp)
 movaps  72160(%rsp), %xmm0
 movaps  72176(%rsp), %xmm1
 movaps  %xmm1, 71040(%rsp)
 movaps  %xmm0, 71024(%rsp)
 movb    $0, 117965(%rsp)
 movaps  68688(%rsp), %xmm0
 movaps  68704(%rsp), %xmm1
 movaps  %xmm1, 72464(%rsp)
 movaps  %xmm0, 72448(%rsp)
 movaps  72448(%rsp), %xmm0
 movaps  72464(%rsp), %xmm1
 movaps  %xmm1, 72432(%rsp)
 movaps  %xmm0, 72416(%rsp)
 movb    $0, 117964(%rsp)
 movaps  69856(%rsp), %xmm0
 movaps  69872(%rsp), %xmm1
 movaps  %xmm1, 72528(%rsp)
 movaps  %xmm0, 72512(%rsp)
 movaps  72512(%rsp), %xmm0
 movaps  72528(%rsp), %xmm1
 movaps  %xmm1, 72496(%rsp)
 movaps  %xmm0, 72480(%rsp)
 movq    $0, 72544(%rsp)
 movb    $0, 117963(%rsp)
 movaps  71024(%rsp), %xmm0
 movaps  71040(%rsp), %xmm1
 movaps  %xmm1, 72592(%rsp)
 movaps  %xmm0, 72576(%rsp)
 movaps  72416(%rsp), %xmm0
 movaps  72432(%rsp), %xmm1
 movups  %xmm1, 72224(%rsp)
 movups  %xmm0, 72208(%rsp)
 movaps  72480(%rsp), %xmm0
 movaps  72496(%rsp), %xmm1
 movups  %xmm1, 72256(%rsp)
 movups  %xmm0, 72240(%rsp)
 movups  72544(%rsp), %xmm0
 movups  72560(%rsp), %xmm1
 movups  %xmm1, 72288(%rsp)
 movups  %xmm0, 72272(%rsp)
 movaps  72576(%rsp), %xmm0
 movaps  72592(%rsp), %xmm1
 movups  %xmm1, 72320(%rsp)
 movups  %xmm0, 72304(%rsp)
 movb    $33, 72200(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    72200(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_950
.LBB7390_946:
 leaq    71064(%rsp), %rdx
 leaq    71648(%rsp), %rcx
 movq    %rcx, 1040(%rsp)
 movl    $256, %r8d
 movq    %r8, 1048(%rsp)
 callq   memcpy
 movq    1040(%rsp), %rdx
 movq    1048(%rsp), %r8
 leaq    71904(%rsp), %rcx
 movq    %rcx, 1056(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1056(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_947
.LBB7390_947:
 movb    $0, 117963(%rsp)
 leaq    69856(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_937
.LBB7390_948:
 jmp     .LBB7390_939
.LBB7390_949:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_948
.LBB7390_950:
 movb    $0, 117963(%rsp)
 movb    $0, 117964(%rsp)
 movb    $0, 117965(%rsp)
 jmp     .LBB7390_906
.LBB7390_951:
 testb   $1, 117963(%rsp)
 jne     .LBB7390_953
 jmp     .LBB7390_939
.LBB7390_952:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_951
.LBB7390_953:
 leaq    71024(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_939
.LBB7390_954:
 leaq    69856(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_926
.LBB7390_955:
 leaq    68688(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_876
.LBB7390_956:
 movq    2840(%rsp), %rax
 movzbl  2(%rax), %eax
 cmpq    $25, %rax
 jne     .LBB7390_872
 movq    2840(%rsp), %rax
 movzbl  3(%rax), %eax
 cmpq    $137, %rax
 jne     .LBB7390_872
 leaq    61536(%rsp), %rcx
 leaq    58312(%rsp), %rdx
 movl    $1, %r8d
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_959
.LBB7390_959:
 leaq    .L__unnamed_400(%rip), %r8
 leaq    61504(%rsp), %rcx
 leaq    61536(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_960
.LBB7390_960:
 movq    2792(%rsp), %r8
 leaq    61240(%rsp), %rcx
 leaq    61504(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_961
.LBB7390_961:
 leaq    60976(%rsp), %rcx
 leaq    61240(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_962
.LBB7390_962:
 movq    60976(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_964
 jmp     .LBB7390_1760
.LBB7390_1760:
 jmp     .LBB7390_965
 ud2
.LBB7390_964:
 movups  60984(%rsp), %xmm0
 movups  61000(%rsp), %xmm1
 movaps  %xmm1, 62096(%rsp)
 movaps  %xmm0, 62080(%rsp)
 movb    $1, 117971(%rsp)
 movaps  62080(%rsp), %xmm0
 movaps  62096(%rsp), %xmm1
 movaps  %xmm1, 60960(%rsp)
 movaps  %xmm0, 60944(%rsp)
 leaq    62704(%rsp), %rcx
 leaq    58312(%rsp), %rdx
 movl    $1, %r8d
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_970
.LBB7390_965:
 leaq    60984(%rsp), %rdx
 leaq    61568(%rsp), %rcx
 movq    %rcx, 1016(%rsp)
 movl    $256, %r8d
 movq    %r8, 1024(%rsp)
 callq   memcpy
 movq    1016(%rsp), %rdx
 movq    1024(%rsp), %r8
 leaq    61824(%rsp), %rcx
 movq    %rcx, 1032(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1032(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_966
.LBB7390_966:
 jmp     .LBB7390_969
.LBB7390_967:
 jmp     .LBB7390_876
.LBB7390_968:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_967
.LBB7390_969:
 movb    $0, 117971(%rsp)
 jmp     .LBB7390_900
.LBB7390_970:
 leaq    .L__unnamed_401(%rip), %r8
 leaq    62672(%rsp), %rcx
 leaq    62704(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_973
.LBB7390_971:
 testb   $1, 117971(%rsp)
 jne     .LBB7390_986
 jmp     .LBB7390_876
.LBB7390_972:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_971
.LBB7390_973:
 movq    2792(%rsp), %r8
 leaq    62408(%rsp), %rcx
 leaq    62672(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_974
.LBB7390_974:
 leaq    62144(%rsp), %rcx
 leaq    62408(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_975
.LBB7390_975:
 movq    62144(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_977
 jmp     .LBB7390_1761
.LBB7390_1761:
 jmp     .LBB7390_978
 ud2
.LBB7390_977:
 movups  62152(%rsp), %xmm0
 movups  62168(%rsp), %xmm1
 movaps  %xmm1, 63264(%rsp)
 movaps  %xmm0, 63248(%rsp)
 movb    $1, 117970(%rsp)
 movaps  63248(%rsp), %xmm0
 movaps  63264(%rsp), %xmm1
 movaps  %xmm1, 62128(%rsp)
 movaps  %xmm0, 62112(%rsp)
 movb    $0, 117971(%rsp)
 movaps  60944(%rsp), %xmm0
 movaps  60960(%rsp), %xmm1
 movaps  %xmm1, 63520(%rsp)
 movaps  %xmm0, 63504(%rsp)
 movb    $0, 117970(%rsp)
 movaps  62112(%rsp), %xmm0
 movaps  62128(%rsp), %xmm1
 movaps  %xmm1, 63552(%rsp)
 movaps  %xmm0, 63536(%rsp)
 movaps  63504(%rsp), %xmm0
 movaps  63520(%rsp), %xmm1
 movups  %xmm1, 63312(%rsp)
 movups  %xmm0, 63296(%rsp)
 movaps  63536(%rsp), %xmm0
 movaps  63552(%rsp), %xmm1
 movups  %xmm1, 63344(%rsp)
 movups  %xmm0, 63328(%rsp)
 movb    $35, 63288(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    63288(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_982
.LBB7390_978:
 leaq    62152(%rsp), %rdx
 leaq    62736(%rsp), %rcx
 movq    %rcx, 992(%rsp)
 movl    $256, %r8d
 movq    %r8, 1000(%rsp)
 callq   memcpy
 movq    992(%rsp), %rdx
 movq    1000(%rsp), %r8
 leaq    62992(%rsp), %rcx
 movq    %rcx, 1008(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    1008(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_979
.LBB7390_979:
 movb    $0, 117970(%rsp)
 leaq    60944(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_969
.LBB7390_980:
 jmp     .LBB7390_971
.LBB7390_981:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_980
.LBB7390_982:
 movb    $0, 117970(%rsp)
 movb    $0, 117971(%rsp)
 jmp     .LBB7390_906
.LBB7390_983:
 testb   $1, 117970(%rsp)
 jne     .LBB7390_985
 jmp     .LBB7390_971
.LBB7390_984:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_983
.LBB7390_985:
 leaq    62112(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_971
.LBB7390_986:
 leaq    60944(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_876
.LBB7390_987:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 cmpq    $13, %rax
 jne     .LBB7390_872
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $133, %rax
 jne     .LBB7390_872
 movq    2840(%rsp), %rax
 movzbl  2(%rax), %eax
 cmpq    $133, %rax
 jne     .LBB7390_872
 movq    2840(%rsp), %rax
 movzbl  3(%rax), %eax
 cmpq    $137, %rax
 jne     .LBB7390_872
 movq    2840(%rsp), %rax
 movzbl  4(%rax), %eax
 cmpq    $131, %rax
 jne     .LBB7390_872
 leaq    64160(%rsp), %rcx
 leaq    58312(%rsp), %rdx
 movl    $1, %r8d
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_993
.LBB7390_993:
 leaq    .L__unnamed_402(%rip), %r8
 leaq    64128(%rsp), %rcx
 leaq    64160(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_994
.LBB7390_994:
 movq    2792(%rsp), %r8
 leaq    63864(%rsp), %rcx
 leaq    64128(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_995
.LBB7390_995:
 leaq    63600(%rsp), %rcx
 leaq    63864(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_996
.LBB7390_996:
 movq    63600(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_998
 jmp     .LBB7390_1752
.LBB7390_1752:
 jmp     .LBB7390_999
 ud2
.LBB7390_998:
 movups  63608(%rsp), %xmm0
 movups  63624(%rsp), %xmm1
 movaps  %xmm1, 64720(%rsp)
 movaps  %xmm0, 64704(%rsp)
 movb    $1, 117969(%rsp)
 movaps  64704(%rsp), %xmm0
 movaps  64720(%rsp), %xmm1
 movaps  %xmm1, 63584(%rsp)
 movaps  %xmm0, 63568(%rsp)
 xorl    %ecx, %ecx
 movl    %ecx, %r8d
 leaq    65328(%rsp), %rcx
 leaq    58312(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_1004
.LBB7390_999:
 leaq    63608(%rsp), %rdx
 leaq    64192(%rsp), %rcx
 movq    %rcx, 968(%rsp)
 movl    $256, %r8d
 movq    %r8, 976(%rsp)
 callq   memcpy
 movq    968(%rsp), %rdx
 movq    976(%rsp), %r8
 leaq    64448(%rsp), %rcx
 movq    %rcx, 984(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    984(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1000
.LBB7390_1000:
 jmp     .LBB7390_1003
.LBB7390_1001:
 jmp     .LBB7390_876
.LBB7390_1002:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1001
.LBB7390_1003:
 movb    $0, 117969(%rsp)
 jmp     .LBB7390_900
.LBB7390_1004:
 leaq    .L__unnamed_403(%rip), %r8
 leaq    65296(%rsp), %rcx
 leaq    65328(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1007
.LBB7390_1005:
 testb   $1, 117969(%rsp)
 jne     .LBB7390_1048
 jmp     .LBB7390_876
.LBB7390_1006:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1005
.LBB7390_1007:
 movq    2792(%rsp), %r8
 leaq    65032(%rsp), %rcx
 leaq    65296(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_1008
.LBB7390_1008:
 leaq    64768(%rsp), %rcx
 leaq    65032(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1009
.LBB7390_1009:
 movq    64768(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1011
 jmp     .LBB7390_1753
.LBB7390_1753:
 jmp     .LBB7390_1012
 ud2
.LBB7390_1011:
 movups  64776(%rsp), %xmm0
 movups  64792(%rsp), %xmm1
 movaps  %xmm1, 65888(%rsp)
 movaps  %xmm0, 65872(%rsp)
 movb    $1, 117968(%rsp)
 movaps  65872(%rsp), %xmm0
 movaps  65888(%rsp), %xmm1
 movaps  %xmm1, 64752(%rsp)
 movaps  %xmm0, 64736(%rsp)
 xorl    %ecx, %ecx
 movl    %ecx, %r8d
 leaq    66496(%rsp), %rcx
 leaq    58312(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_1017
.LBB7390_1012:
 leaq    64776(%rsp), %rdx
 leaq    65360(%rsp), %rcx
 movq    %rcx, 944(%rsp)
 movl    $256, %r8d
 movq    %r8, 952(%rsp)
 callq   memcpy
 movq    944(%rsp), %rdx
 movq    952(%rsp), %r8
 leaq    65616(%rsp), %rcx
 movq    %rcx, 960(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    960(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1013
.LBB7390_1013:
 jmp     .LBB7390_1016
.LBB7390_1014:
 jmp     .LBB7390_1005
.LBB7390_1015:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1014
.LBB7390_1016:
 movb    $0, 117968(%rsp)
 leaq    63568(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1003
.LBB7390_1017:
 leaq    .L__unnamed_404(%rip), %r8
 leaq    66464(%rsp), %rcx
 leaq    66496(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1020
.LBB7390_1018:
 testb   $1, 117968(%rsp)
 jne     .LBB7390_1047
 jmp     .LBB7390_1005
.LBB7390_1019:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1018
.LBB7390_1020:
 movq    2792(%rsp), %r8
 leaq    66200(%rsp), %rcx
 leaq    66464(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_1021
.LBB7390_1021:
 leaq    65936(%rsp), %rcx
 leaq    66200(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1022
.LBB7390_1022:
 movq    65936(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1024
 jmp     .LBB7390_1754
.LBB7390_1754:
 jmp     .LBB7390_1025
 ud2
.LBB7390_1024:
 movups  65944(%rsp), %xmm0
 movups  65960(%rsp), %xmm1
 movaps  %xmm1, 67056(%rsp)
 movaps  %xmm0, 67040(%rsp)
 movb    $1, 117967(%rsp)
 movaps  67040(%rsp), %xmm0
 movaps  67056(%rsp), %xmm1
 movaps  %xmm1, 65920(%rsp)
 movaps  %xmm0, 65904(%rsp)
 xorl    %ecx, %ecx
 movl    %ecx, %r8d
 leaq    67664(%rsp), %rcx
 leaq    58312(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_1030
.LBB7390_1025:
 leaq    65944(%rsp), %rdx
 leaq    66528(%rsp), %rcx
 movq    %rcx, 920(%rsp)
 movl    $256, %r8d
 movq    %r8, 928(%rsp)
 callq   memcpy
 movq    920(%rsp), %rdx
 movq    928(%rsp), %r8
 leaq    66784(%rsp), %rcx
 movq    %rcx, 936(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    936(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1026
.LBB7390_1026:
 jmp     .LBB7390_1029
.LBB7390_1027:
 jmp     .LBB7390_1018
.LBB7390_1028:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1027
.LBB7390_1029:
 movb    $0, 117967(%rsp)
 leaq    64736(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1016
.LBB7390_1030:
 leaq    .L__unnamed_405(%rip), %r8
 leaq    67632(%rsp), %rcx
 leaq    67664(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1033
.LBB7390_1031:
 testb   $1, 117967(%rsp)
 jne     .LBB7390_1046
 jmp     .LBB7390_1018
.LBB7390_1032:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1031
.LBB7390_1033:
 movq    2792(%rsp), %r8
 leaq    67368(%rsp), %rcx
 leaq    67632(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_1034
.LBB7390_1034:
 leaq    67104(%rsp), %rcx
 leaq    67368(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1035
.LBB7390_1035:
 movq    67104(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1037
 jmp     .LBB7390_1755
.LBB7390_1755:
 jmp     .LBB7390_1038
 ud2
.LBB7390_1037:
 movups  67112(%rsp), %xmm0
 movups  67128(%rsp), %xmm1
 movaps  %xmm1, 68224(%rsp)
 movaps  %xmm0, 68208(%rsp)
 movb    $1, 117966(%rsp)
 movaps  68208(%rsp), %xmm0
 movaps  68224(%rsp), %xmm1
 movaps  %xmm1, 67088(%rsp)
 movaps  %xmm0, 67072(%rsp)
 movb    $0, 117969(%rsp)
 movaps  63568(%rsp), %xmm0
 movaps  63584(%rsp), %xmm1
 movaps  %xmm1, 68512(%rsp)
 movaps  %xmm0, 68496(%rsp)
 movaps  68496(%rsp), %xmm0
 movaps  68512(%rsp), %xmm1
 movaps  %xmm1, 68480(%rsp)
 movaps  %xmm0, 68464(%rsp)
 movb    $0, 117968(%rsp)
 movaps  64736(%rsp), %xmm0
 movaps  64752(%rsp), %xmm1
 movaps  %xmm1, 68576(%rsp)
 movaps  %xmm0, 68560(%rsp)
 movaps  68560(%rsp), %xmm0
 movaps  68576(%rsp), %xmm1
 movaps  %xmm1, 68544(%rsp)
 movaps  %xmm0, 68528(%rsp)
 movb    $0, 117967(%rsp)
 movaps  65904(%rsp), %xmm0
 movaps  65920(%rsp), %xmm1
 movaps  %xmm1, 68640(%rsp)
 movaps  %xmm0, 68624(%rsp)
 movaps  68624(%rsp), %xmm0
 movaps  68640(%rsp), %xmm1
 movaps  %xmm1, 68608(%rsp)
 movaps  %xmm0, 68592(%rsp)
 movb    $0, 117966(%rsp)
 movaps  67072(%rsp), %xmm0
 movaps  67088(%rsp), %xmm1
 movaps  %xmm1, 68672(%rsp)
 movaps  %xmm0, 68656(%rsp)
 movaps  68464(%rsp), %xmm0
 movaps  68480(%rsp), %xmm1
 movups  %xmm1, 68272(%rsp)
 movups  %xmm0, 68256(%rsp)
 movaps  68528(%rsp), %xmm0
 movaps  68544(%rsp), %xmm1
 movups  %xmm1, 68304(%rsp)
 movups  %xmm0, 68288(%rsp)
 movaps  68592(%rsp), %xmm0
 movaps  68608(%rsp), %xmm1
 movups  %xmm1, 68336(%rsp)
 movups  %xmm0, 68320(%rsp)
 movaps  68656(%rsp), %xmm0
 movaps  68672(%rsp), %xmm1
 movups  %xmm1, 68368(%rsp)
 movups  %xmm0, 68352(%rsp)
 movb    $33, 68248(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    68248(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_1042
.LBB7390_1038:
 leaq    67112(%rsp), %rdx
 leaq    67696(%rsp), %rcx
 movq    %rcx, 896(%rsp)
 movl    $256, %r8d
 movq    %r8, 904(%rsp)
 callq   memcpy
 movq    896(%rsp), %rdx
 movq    904(%rsp), %r8
 leaq    67952(%rsp), %rcx
 movq    %rcx, 912(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    912(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1039
.LBB7390_1039:
 movb    $0, 117966(%rsp)
 leaq    65904(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1029
.LBB7390_1040:
 jmp     .LBB7390_1031
.LBB7390_1041:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1040
.LBB7390_1042:
 movb    $0, 117966(%rsp)
 movb    $0, 117967(%rsp)
 movb    $0, 117968(%rsp)
 movb    $0, 117969(%rsp)
 jmp     .LBB7390_906
.LBB7390_1043:
 testb   $1, 117966(%rsp)
 jne     .LBB7390_1045
 jmp     .LBB7390_1031
.LBB7390_1044:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1043
.LBB7390_1045:
 leaq    67072(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1031
.LBB7390_1046:
 leaq    65904(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1018
.LBB7390_1047:
 leaq    64736(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1005
.LBB7390_1048:
 leaq    63568(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_876
.LBB7390_1049:
 movq    1168(%rsp), %rcx
 movq    1176(%rsp), %rdx
 movq    %rdx, 72664(%rsp)
 movq    %rcx, 72672(%rsp)
 movq    %rsp, %rcx
 movq    $1, 32(%rcx)
 leaq    .L__unnamed_406(%rip), %rdx
 leaq    72616(%rsp), %rcx
 movl    $1, %r8d
 leaq    72664(%rsp), %r9
 callq   core::fmt::Arguments::new_v1
 jmp     .LBB7390_1050
.LBB7390_1050:
 leaq    .L__unnamed_407(%rip), %rdx
 leaq    72616(%rsp), %rcx
 callq   std::panicking::begin_panic_fmt
 jmp     .LBB7390_9
.LBB7390_1051:
 leaq    .L__unnamed_408(%rip), %r8
 leaq    47200(%rsp), %rcx
 leaq    47232(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1054
.LBB7390_1052:
 testb   $1, 117979(%rsp)
 jne     .LBB7390_1191
 jmp     .LBB7390_1190
.LBB7390_1053:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1052
.LBB7390_1054:
 leaq    47200(%rsp), %rcx
 callq   pest::iterators::pair::Pair<R>::as_rule
 movb    %al, %cl
 movb    %cl, 895(%rsp)
 jmp     .LBB7390_1055
.LBB7390_1055:
 movb    895(%rsp), %cl
 movb    %cl, 47199(%rsp)
 leaq    47200(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1058
.LBB7390_1056:
 leaq    47200(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1052
.LBB7390_1057:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1056
.LBB7390_1058:
 movb    $0, 117979(%rsp)
 movq    47184(%rsp), %rcx
 movq    32(%rcx), %rdx
 movq    %rdx, 47344(%rsp)
 movups  (%rcx), %xmm0
 movups  16(%rcx), %xmm1
 movaps  %xmm1, 47328(%rsp)
 movaps  %xmm0, 47312(%rsp)
 leaq    47264(%rsp), %rcx
 leaq    47312(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::new
 jmp     .LBB7390_1059
.LBB7390_1059:
 movzbl  47199(%rsp), %eax
 movq    %rax, 880(%rsp)
 subq    $14, %rax
 je      .LBB7390_1061
 jmp     .LBB7390_1764
.LBB7390_1764:
 movq    880(%rsp), %rax
 subq    $45, %rax
 je      .LBB7390_1062
 jmp     .LBB7390_1060
.LBB7390_1060:
 leaq    .L__unnamed_181(%rip), %rcx
 leaq    .L__unnamed_409(%rip), %r8
 movl    $40, %edx
 callq   core::panicking::panic
 jmp     .LBB7390_9
.LBB7390_1061:
 movb    $-119, 48495(%rsp)
 movb    48495(%rsp), %r8b
 leaq    48224(%rsp), %rcx
 leaq    47264(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_1127
.LBB7390_1062:
 movb    $-119, 54447(%rsp)
 movb    54447(%rsp), %r8b
 leaq    54176(%rsp), %rcx
 leaq    47264(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_1065
.LBB7390_1063:
 leaq    47264(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_1052
.LBB7390_1064:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1063
.LBB7390_1065:
 leaq    53912(%rsp), %rcx
 leaq    54176(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1066
.LBB7390_1066:
 movq    53912(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1068
 jmp     .LBB7390_1765
.LBB7390_1765:
 jmp     .LBB7390_1069
 ud2
.LBB7390_1068:
 movq    2792(%rsp), %r8
 movups  53920(%rsp), %xmm0
 movups  53936(%rsp), %xmm1
 movaps  %xmm1, 54976(%rsp)
 movaps  %xmm0, 54960(%rsp)
 movaps  54960(%rsp), %xmm0
 movaps  54976(%rsp), %xmm1
 movaps  %xmm1, 53888(%rsp)
 movaps  %xmm0, 53872(%rsp)
 leaq    53608(%rsp), %rcx
 leaq    53872(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_1074
.LBB7390_1069:
 leaq    53920(%rsp), %rdx
 leaq    54448(%rsp), %rcx
 movq    %rcx, 856(%rsp)
 movl    $256, %r8d
 movq    %r8, 864(%rsp)
 callq   memcpy
 movq    856(%rsp), %rdx
 movq    864(%rsp), %r8
 leaq    54704(%rsp), %rcx
 movq    %rcx, 872(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    872(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1070
.LBB7390_1070:
 jmp     .LBB7390_1073
.LBB7390_1071:
 jmp     .LBB7390_1063
.LBB7390_1072:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1071
.LBB7390_1073:
 jmp     .LBB7390_1082
.LBB7390_1074:
 leaq    53344(%rsp), %rcx
 leaq    53608(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1075
.LBB7390_1075:
 movq    53344(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1077
 jmp     .LBB7390_1766
.LBB7390_1766:
 jmp     .LBB7390_1078
 ud2
.LBB7390_1077:
 movups  53352(%rsp), %xmm0
 movups  53368(%rsp), %xmm1
 movaps  %xmm1, 55520(%rsp)
 movaps  %xmm0, 55504(%rsp)
 movb    $1, 117976(%rsp)
 movaps  55504(%rsp), %xmm0
 movaps  55520(%rsp), %xmm1
 movaps  %xmm1, 53328(%rsp)
 movaps  %xmm0, 53312(%rsp)
 leaq    55536(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1083
.LBB7390_1078:
 leaq    53352(%rsp), %rdx
 leaq    54992(%rsp), %rcx
 movq    %rcx, 832(%rsp)
 movl    $256, %r8d
 movq    %r8, 840(%rsp)
 callq   memcpy
 movq    832(%rsp), %rdx
 movq    840(%rsp), %r8
 leaq    55248(%rsp), %rcx
 movq    %rcx, 848(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    848(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1079
.LBB7390_1079:
 jmp     .LBB7390_1073
.LBB7390_1080:
 jmp     .LBB7390_1071
.LBB7390_1081:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1080
.LBB7390_1082:
 movb    $0, 117976(%rsp)
 jmp     .LBB7390_1117
.LBB7390_1083:
 movb    $1, 117975(%rsp)
 movb    $-124, 55567(%rsp)
 movb    55567(%rsp), %dl
 leaq    47264(%rsp), %rcx
 callq   jodin_rs::ast::IndexedPair::contains
 movb    %al, %cl
 movb    %cl, 831(%rsp)
 jmp     .LBB7390_1086
.LBB7390_1084:
 testb   $1, 117976(%rsp)
 jne     .LBB7390_1125
 jmp     .LBB7390_1063
.LBB7390_1085:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1084
.LBB7390_1086:
 movb    831(%rsp), %al
 testb   $1, %al
 jne     .LBB7390_1090
 jmp     .LBB7390_1089
.LBB7390_1087:
 testb   $1, 117975(%rsp)
 jne     .LBB7390_1124
 jmp     .LBB7390_1084
.LBB7390_1088:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1087
.LBB7390_1089:
 jmp     .LBB7390_1122
.LBB7390_1090:
 movb    $-124, 56159(%rsp)
 movb    56159(%rsp), %r8b
 leaq    55888(%rsp), %rcx
 leaq    47264(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get_all
 jmp     .LBB7390_1091
.LBB7390_1091:
 leaq    55624(%rsp), %rcx
 leaq    55888(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1092
.LBB7390_1092:
 movq    55624(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1094
 jmp     .LBB7390_1767
.LBB7390_1767:
 jmp     .LBB7390_1095
 ud2
.LBB7390_1094:
 movq    55648(%rsp), %rcx
 movq    %rcx, 56688(%rsp)
 movups  55632(%rsp), %xmm0
 movaps  %xmm0, 56672(%rsp)
 movq    56688(%rsp), %rcx
 movq    %rcx, 55616(%rsp)
 movaps  56672(%rsp), %xmm0
 movaps  %xmm0, 55600(%rsp)
 leaq    55568(%rsp), %rcx
 leaq    55600(%rsp), %rdx
 callq   <alloc::vec::Vec<T,A> as core::iter::traits::collect::IntoIterator>::into_iter
 jmp     .LBB7390_1100
.LBB7390_1095:
 leaq    55632(%rsp), %rdx
 leaq    56160(%rsp), %rcx
 movq    %rcx, 800(%rsp)
 movl    $256, %r8d
 movq    %r8, 808(%rsp)
 callq   memcpy
 movq    800(%rsp), %rdx
 movq    808(%rsp), %r8
 leaq    56416(%rsp), %rcx
 movq    %rcx, 816(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    816(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1096
.LBB7390_1096:
 jmp     .LBB7390_1099
.LBB7390_1097:
 jmp     .LBB7390_1087
.LBB7390_1098:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1097
.LBB7390_1099:
 leaq    55536(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_1116
.LBB7390_1100:
 movq    55568(%rsp), %rax
 movq    %rax, 56704(%rsp)
 movq    55576(%rsp), %rax
 movq    %rax, 56712(%rsp)
 movq    55584(%rsp), %rax
 movq    %rax, 56720(%rsp)
 movq    55592(%rsp), %rax
 movq    %rax, 56728(%rsp)
.LBB7390_1101:
 leaq    56768(%rsp), %rcx
 leaq    56704(%rsp), %rdx
 callq   <alloc::vec::into_iter::IntoIter<T,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1102
.LBB7390_1102:
 movq    56768(%rsp), %rax
 testq   %rax, %rax
 setne   %al
 movzbl  %al, %eax
 je      .LBB7390_1106
 jmp     .LBB7390_1768
.LBB7390_1768:
 jmp     .LBB7390_1107
.LBB7390_1103:
 testb   $1, 117974(%rsp)
 jne     .LBB7390_1120
 jmp     .LBB7390_1119
.LBB7390_1104:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1103
 ud2
.LBB7390_1106:
 movb    $0, 117974(%rsp)
 leaq    56704(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1121
.LBB7390_1107:
 movq    2792(%rsp), %r8
 movups  56768(%rsp), %xmm0
 movups  56784(%rsp), %xmm1
 movaps  %xmm1, 56816(%rsp)
 movaps  %xmm0, 56800(%rsp)
 movaps  56800(%rsp), %xmm0
 movaps  56816(%rsp), %xmm1
 movaps  %xmm1, 56848(%rsp)
 movaps  %xmm0, 56832(%rsp)
 movb    $1, 117974(%rsp)
 movaps  56832(%rsp), %xmm0
 movaps  56848(%rsp), %xmm1
 movaps  %xmm1, 56752(%rsp)
 movaps  %xmm0, 56736(%rsp)
 movb    $0, 117974(%rsp)
 movaps  56736(%rsp), %xmm0
 movaps  56752(%rsp), %xmm1
 movaps  %xmm1, 56880(%rsp)
 movaps  %xmm0, 56864(%rsp)
 movaps  56864(%rsp), %xmm0
 movaps  56880(%rsp), %xmm1
 movaps  %xmm1, 57472(%rsp)
 movaps  %xmm0, 57456(%rsp)
 leaq    57192(%rsp), %rcx
 leaq    57456(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_1108
.LBB7390_1108:
 leaq    56928(%rsp), %rcx
 leaq    57192(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1109
.LBB7390_1109:
 movq    56928(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1111
 jmp     .LBB7390_1769
.LBB7390_1769:
 jmp     .LBB7390_1112
 ud2
.LBB7390_1111:
 movups  56936(%rsp), %xmm0
 movups  56952(%rsp), %xmm1
 movaps  %xmm1, 58016(%rsp)
 movaps  %xmm0, 58000(%rsp)
 movaps  58000(%rsp), %xmm0
 movaps  58016(%rsp), %xmm1
 movaps  %xmm1, 56912(%rsp)
 movaps  %xmm0, 56896(%rsp)
 leaq    55536(%rsp), %rcx
 leaq    56896(%rsp), %rdx
 callq   alloc::vec::Vec<T,A>::push
 jmp     .LBB7390_1118
.LBB7390_1112:
 leaq    56936(%rsp), %rdx
 leaq    57488(%rsp), %rcx
 movq    %rcx, 776(%rsp)
 movl    $256, %r8d
 movq    %r8, 784(%rsp)
 callq   memcpy
 movq    776(%rsp), %rdx
 movq    784(%rsp), %r8
 leaq    57744(%rsp), %rcx
 movq    %rcx, 792(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    792(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1113
.LBB7390_1113:
 movb    $0, 117974(%rsp)
 leaq    56704(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1099
.LBB7390_1114:
 jmp     .LBB7390_1103
.LBB7390_1115:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1114
.LBB7390_1116:
 movb    $0, 117975(%rsp)
 leaq    53312(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1082
.LBB7390_1117:
 leaq    47264(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_1182
.LBB7390_1118:
 movb    $0, 117974(%rsp)
 jmp     .LBB7390_1101
.LBB7390_1119:
 leaq    56704(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1097
.LBB7390_1120:
 leaq    56736(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1119
.LBB7390_1121:
 jmp     .LBB7390_1122
.LBB7390_1122:
 movb    $0, 117976(%rsp)
 movaps  53312(%rsp), %xmm0
 movaps  53328(%rsp), %xmm1
 movaps  %xmm1, 58272(%rsp)
 movaps  %xmm0, 58256(%rsp)
 movb    $0, 117975(%rsp)
 movq    55552(%rsp), %rcx
 movq    %rcx, 58304(%rsp)
 movups  55536(%rsp), %xmm0
 movaps  %xmm0, 58288(%rsp)
 movaps  58256(%rsp), %xmm0
 movaps  58272(%rsp), %xmm1
 movups  %xmm1, 58064(%rsp)
 movups  %xmm0, 58048(%rsp)
 movq    58304(%rsp), %rcx
 movq    %rcx, 58096(%rsp)
 movaps  58288(%rsp), %xmm0
 movups  %xmm0, 58080(%rsp)
 movb    $34, 58040(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    58040(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_1123
.LBB7390_1123:
 movb    $0, 117975(%rsp)
 movb    $0, 117976(%rsp)
 jmp     .LBB7390_1126
.LBB7390_1124:
 leaq    55536(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_1084
.LBB7390_1125:
 leaq    53312(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1063
.LBB7390_1126:
 leaq    47264(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_1189
.LBB7390_1127:
 leaq    47960(%rsp), %rcx
 leaq    48224(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1128
.LBB7390_1128:
 movq    47960(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1130
 jmp     .LBB7390_1770
.LBB7390_1770:
 jmp     .LBB7390_1131
 ud2
.LBB7390_1130:
 movq    2792(%rsp), %r8
 movups  47968(%rsp), %xmm0
 movups  47984(%rsp), %xmm1
 movaps  %xmm1, 49024(%rsp)
 movaps  %xmm0, 49008(%rsp)
 movaps  49008(%rsp), %xmm0
 movaps  49024(%rsp), %xmm1
 movaps  %xmm1, 47936(%rsp)
 movaps  %xmm0, 47920(%rsp)
 leaq    47656(%rsp), %rcx
 leaq    47920(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_1136
.LBB7390_1131:
 leaq    47968(%rsp), %rdx
 leaq    48496(%rsp), %rcx
 movq    %rcx, 752(%rsp)
 movl    $256, %r8d
 movq    %r8, 760(%rsp)
 callq   memcpy
 movq    752(%rsp), %rdx
 movq    760(%rsp), %r8
 leaq    48752(%rsp), %rcx
 movq    %rcx, 768(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    768(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1132
.LBB7390_1132:
 jmp     .LBB7390_1135
.LBB7390_1133:
 jmp     .LBB7390_1063
.LBB7390_1134:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1133
.LBB7390_1135:
 jmp     .LBB7390_1144
.LBB7390_1136:
 leaq    47392(%rsp), %rcx
 leaq    47656(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1137
.LBB7390_1137:
 movq    47392(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1139
 jmp     .LBB7390_1771
.LBB7390_1771:
 jmp     .LBB7390_1140
 ud2
.LBB7390_1139:
 movups  47400(%rsp), %xmm0
 movups  47416(%rsp), %xmm1
 movaps  %xmm1, 49568(%rsp)
 movaps  %xmm0, 49552(%rsp)
 movb    $1, 117978(%rsp)
 movaps  49552(%rsp), %xmm0
 movaps  49568(%rsp), %xmm1
 movaps  %xmm1, 47376(%rsp)
 movaps  %xmm0, 47360(%rsp)
 movb    $-125, 50143(%rsp)
 movb    50143(%rsp), %r8b
 leaq    49872(%rsp), %rcx
 leaq    47264(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get_all
 jmp     .LBB7390_1145
.LBB7390_1140:
 leaq    47400(%rsp), %rdx
 leaq    49040(%rsp), %rcx
 movq    %rcx, 728(%rsp)
 movl    $256, %r8d
 movq    %r8, 736(%rsp)
 callq   memcpy
 movq    728(%rsp), %rdx
 movq    736(%rsp), %r8
 leaq    49296(%rsp), %rcx
 movq    %rcx, 744(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    744(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1141
.LBB7390_1141:
 jmp     .LBB7390_1135
.LBB7390_1142:
 jmp     .LBB7390_1133
.LBB7390_1143:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1142
.LBB7390_1144:
 movb    $0, 117978(%rsp)
 jmp     .LBB7390_1117
.LBB7390_1145:
 leaq    49608(%rsp), %rcx
 leaq    49872(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1148
.LBB7390_1146:
 testb   $1, 117978(%rsp)
 jne     .LBB7390_1188
 jmp     .LBB7390_1063
.LBB7390_1147:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1146
.LBB7390_1148:
 movq    49608(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1150
 jmp     .LBB7390_1772
.LBB7390_1772:
 jmp     .LBB7390_1151
 ud2
.LBB7390_1150:
 movq    49632(%rsp), %rcx
 movq    %rcx, 50672(%rsp)
 movups  49616(%rsp), %xmm0
 movaps  %xmm0, 50656(%rsp)
 movq    50672(%rsp), %rcx
 movq    %rcx, 49600(%rsp)
 movaps  50656(%rsp), %xmm0
 movaps  %xmm0, 49584(%rsp)
 xorl    %ecx, %ecx
 movl    %ecx, %r8d
 leaq    51248(%rsp), %rcx
 leaq    49584(%rsp), %rdx
 callq   alloc::vec::Vec<T,A>::remove
 jmp     .LBB7390_1156
.LBB7390_1151:
 leaq    49616(%rsp), %rdx
 leaq    50144(%rsp), %rcx
 movq    %rcx, 704(%rsp)
 movl    $256, %r8d
 movq    %r8, 712(%rsp)
 callq   memcpy
 movq    704(%rsp), %rdx
 movq    712(%rsp), %r8
 leaq    50400(%rsp), %rcx
 movq    %rcx, 720(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    720(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1152
.LBB7390_1152:
 jmp     .LBB7390_1155
.LBB7390_1153:
 jmp     .LBB7390_1146
.LBB7390_1154:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1153
.LBB7390_1155:
 leaq    47360(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1144
.LBB7390_1156:
 movq    2792(%rsp), %r8
 leaq    50984(%rsp), %rcx
 leaq    51248(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_1159
.LBB7390_1157:
 leaq    49584(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1146
.LBB7390_1158:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1157
.LBB7390_1159:
 leaq    50720(%rsp), %rcx
 leaq    50984(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1160
.LBB7390_1160:
 movq    50720(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1162
 jmp     .LBB7390_1773
.LBB7390_1773:
 jmp     .LBB7390_1163
 ud2
.LBB7390_1162:
 movups  50728(%rsp), %xmm0
 movups  50744(%rsp), %xmm1
 movaps  %xmm1, 51808(%rsp)
 movaps  %xmm0, 51792(%rsp)
 movb    $1, 117977(%rsp)
 movaps  51792(%rsp), %xmm0
 movaps  51808(%rsp), %xmm1
 movaps  %xmm1, 50704(%rsp)
 movaps  %xmm0, 50688(%rsp)
 movb    $11, 51863(%rsp)
 movb    51863(%rsp), %dl
 leaq    47264(%rsp), %rcx
 callq   jodin_rs::ast::IndexedPair::contains
 movb    %al, %cl
 movb    %cl, 703(%rsp)
 jmp     .LBB7390_1168
.LBB7390_1163:
 leaq    50728(%rsp), %rdx
 leaq    51280(%rsp), %rcx
 movq    %rcx, 672(%rsp)
 movl    $256, %r8d
 movq    %r8, 680(%rsp)
 callq   memcpy
 movq    672(%rsp), %rdx
 movq    680(%rsp), %r8
 leaq    51536(%rsp), %rcx
 movq    %rcx, 688(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    688(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1164
.LBB7390_1164:
 jmp     .LBB7390_1167
.LBB7390_1165:
 jmp     .LBB7390_1157
.LBB7390_1166:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1165
.LBB7390_1167:
 movb    $0, 117977(%rsp)
 leaq    49584(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1155
.LBB7390_1168:
 movb    703(%rsp), %al
 testb   $1, %al
 jne     .LBB7390_1172
 jmp     .LBB7390_1171
.LBB7390_1169:
 testb   $1, 117977(%rsp)
 jne     .LBB7390_1186
 jmp     .LBB7390_1157
.LBB7390_1170:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1169
.LBB7390_1171:
 movq    $0, 51824(%rsp)
 jmp     .LBB7390_1184
.LBB7390_1172:
 xorl    %ecx, %ecx
 movl    %ecx, %r8d
 leaq    52424(%rsp), %rcx
 leaq    49584(%rsp), %rdx
 callq   alloc::vec::Vec<T,A>::remove
 jmp     .LBB7390_1173
.LBB7390_1173:
 movq    2792(%rsp), %r8
 leaq    52160(%rsp), %rcx
 leaq    52424(%rsp), %rdx
 callq   <pest::iterators::pair::Pair<jodin_rs::parsing::Rule> as jodin_rs::ast::NodeExtension>::generate_node
 jmp     .LBB7390_1174
.LBB7390_1174:
 leaq    51896(%rsp), %rcx
 leaq    52160(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1175
.LBB7390_1175:
 movq    51896(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1177
 jmp     .LBB7390_1774
.LBB7390_1774:
 jmp     .LBB7390_1178
 ud2
.LBB7390_1177:
 movq    51904(%rsp), %rax
 movq    %rax, 52968(%rsp)
 movq    51912(%rsp), %rax
 movq    %rax, 52976(%rsp)
 movq    51920(%rsp), %rax
 movq    %rax, 52984(%rsp)
 movq    51928(%rsp), %rax
 movq    %rax, 52992(%rsp)
 movq    52968(%rsp), %rax
 movq    %rax, 51864(%rsp)
 movq    52976(%rsp), %rax
 movq    %rax, 51872(%rsp)
 movq    52984(%rsp), %rax
 movq    %rax, 51880(%rsp)
 movq    52992(%rsp), %rax
 movq    %rax, 51888(%rsp)
 movq    51864(%rsp), %rax
 movq    %rax, 51824(%rsp)
 movq    51872(%rsp), %rax
 movq    %rax, 51832(%rsp)
 movq    51880(%rsp), %rax
 movq    %rax, 51840(%rsp)
 movq    51888(%rsp), %rax
 movq    %rax, 51848(%rsp)
 jmp     .LBB7390_1184
.LBB7390_1178:
 leaq    51904(%rsp), %rdx
 leaq    52456(%rsp), %rcx
 movq    %rcx, 648(%rsp)
 movl    $256, %r8d
 movq    %r8, 656(%rsp)
 callq   memcpy
 movq    648(%rsp), %rdx
 movq    656(%rsp), %r8
 leaq    52712(%rsp), %rcx
 movq    %rcx, 664(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    664(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1179
.LBB7390_1179:
 leaq    50688(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1167
.LBB7390_1180:
 jmp     .LBB7390_1169
.LBB7390_1181:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1180
.LBB7390_1182:
 movq    47184(%rsp), %rcx
 callq   alloc::alloc::box_free
 movb    $0, 117979(%rsp)
 jmp     .LBB7390_104
.LBB7390_1184:
 movb    $0, 117978(%rsp)
 movaps  47360(%rsp), %xmm0
 movaps  47376(%rsp), %xmm1
 movaps  %xmm1, 53232(%rsp)
 movaps  %xmm0, 53216(%rsp)
 movb    $0, 117977(%rsp)
 movaps  50688(%rsp), %xmm0
 movaps  50704(%rsp), %xmm1
 movaps  %xmm1, 53264(%rsp)
 movaps  %xmm0, 53248(%rsp)
 movups  51824(%rsp), %xmm0
 movups  51840(%rsp), %xmm1
 movaps  %xmm1, 53296(%rsp)
 movaps  %xmm0, 53280(%rsp)
 movaps  53216(%rsp), %xmm0
 movaps  53232(%rsp), %xmm1
 movups  %xmm1, 53024(%rsp)
 movups  %xmm0, 53008(%rsp)
 movaps  53248(%rsp), %xmm0
 movaps  53264(%rsp), %xmm1
 movups  %xmm1, 53056(%rsp)
 movups  %xmm0, 53040(%rsp)
 movaps  53280(%rsp), %xmm0
 movaps  53296(%rsp), %xmm1
 movups  %xmm1, 53088(%rsp)
 movups  %xmm0, 53072(%rsp)
 movb    $31, 53000(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    53000(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_1185
.LBB7390_1185:
 movb    $0, 117977(%rsp)
 leaq    49584(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1187
.LBB7390_1186:
 leaq    50688(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1157
.LBB7390_1187:
 movb    $0, 117978(%rsp)
 jmp     .LBB7390_1126
.LBB7390_1188:
 leaq    47360(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1063
.LBB7390_1189:
 movq    47184(%rsp), %rcx
 callq   alloc::alloc::box_free
 jmp     .LBB7390_1192
.LBB7390_1190:
 movq    47184(%rsp), %rcx
 callq   alloc::alloc::box_free
 jmp     .LBB7390_85
.LBB7390_1191:
 movq    47184(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1190
.LBB7390_1192:
 movb    $0, 117979(%rsp)
 jmp     .LBB7390_89
.LBB7390_1193:
 cmpq    $1, 2848(%rsp)
 je      .LBB7390_1218
 jmp     .LBB7390_1196
.LBB7390_1194:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 cmpq    $137, %rax
 jne     .LBB7390_1196
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $87, %rax
 je      .LBB7390_1197
.LBB7390_1196:
 leaq    2840(%rsp), %rax
 movq    %rax, 47176(%rsp)
 movq    47176(%rsp), %rcx
 movq    %rcx, 118184(%rsp)
 leaq    _ZN67_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17he8271684dacd979dE(%rip), %rdx
 callq   core::fmt::ArgumentV1::new
 movq    %rax, %rcx
 movq    %rdx, %r8
 movq    %r8, 632(%rsp)
 movq    %rcx, 640(%rsp)
 jmp     .LBB7390_1221
.LBB7390_1197:
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %rcx
 movq    %rcx, 45688(%rsp)
 xorl    %ecx, %ecx
 movl    %ecx, %r8d
 leaq    45728(%rsp), %rcx
 leaq    45688(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::nth
 jmp     .LBB7390_1198
.LBB7390_1198:
 leaq    .L__unnamed_410(%rip), %r8
 leaq    45696(%rsp), %rcx
 leaq    45728(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1201
.LBB7390_1199:
 leaq    45688(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_85
.LBB7390_1200:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1199
.LBB7390_1201:
 movb    $1, 117980(%rsp)
 movups  45696(%rsp), %xmm0
 movups  45712(%rsp), %xmm1
 movaps  %xmm1, 46304(%rsp)
 movaps  %xmm0, 46288(%rsp)
 leaq    46328(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1202
.LBB7390_1202:
 movq    2792(%rsp), %rdx
 movb    $0, 117980(%rsp)
 leaq    46024(%rsp), %rcx
 leaq    46288(%rsp), %r8
 leaq    46328(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_1205
.LBB7390_1203:
 testb   $1, 117980(%rsp)
 jne     .LBB7390_1206
 jmp     .LBB7390_1199
.LBB7390_1204:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1203
.LBB7390_1205:
 movb    $0, 117980(%rsp)
 leaq    45760(%rsp), %rcx
 leaq    46024(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1207
.LBB7390_1206:
 leaq    46288(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1199
.LBB7390_1207:
 movq    45760(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1209
 jmp     .LBB7390_1775
.LBB7390_1775:
 jmp     .LBB7390_1210
 ud2
.LBB7390_1209:
 movups  45768(%rsp), %xmm0
 movups  45784(%rsp), %xmm1
 movaps  %xmm1, 46880(%rsp)
 movaps  %xmm0, 46864(%rsp)
 movaps  46864(%rsp), %xmm0
 movaps  46880(%rsp), %xmm1
 movaps  %xmm1, 3392(%rsp)
 movaps  %xmm0, 3376(%rsp)
 leaq    45688(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1217
.LBB7390_1210:
 leaq    45768(%rsp), %rdx
 leaq    46352(%rsp), %rcx
 movq    %rcx, 608(%rsp)
 movl    $256, %r8d
 movq    %r8, 616(%rsp)
 callq   memcpy
 movq    608(%rsp), %rdx
 movq    616(%rsp), %r8
 leaq    46608(%rsp), %rcx
 movq    %rcx, 624(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    624(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1211
.LBB7390_1211:
 leaq    45688(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1215
.LBB7390_1212:
 leaq    45688(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1214
.LBB7390_1213:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1212
.LBB7390_1214:
 jmp     .LBB7390_85
.LBB7390_1215:
 jmp     .LBB7390_104
.LBB7390_1216:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1214
.LBB7390_1217:
 jmp     .LBB7390_89
.LBB7390_1218:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 cmpq    $87, %rax
 jne     .LBB7390_1196
 movb    $24, 46896(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    46896(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_1220
.LBB7390_1220:
 jmp     .LBB7390_89
.LBB7390_1221:
 movq    632(%rsp), %rcx
 movq    640(%rsp), %rdx
 movq    %rdx, 47160(%rsp)
 movq    %rcx, 47168(%rsp)
 movq    %rsp, %rcx
 movq    $1, 32(%rcx)
 leaq    .L__unnamed_393(%rip), %rdx
 leaq    47112(%rsp), %rcx
 movl    $1, %r8d
 leaq    47160(%rsp), %r9
 callq   core::fmt::Arguments::new_v1
 jmp     .LBB7390_1222
.LBB7390_1222:
 leaq    .L__unnamed_411(%rip), %rdx
 leaq    47112(%rsp), %rcx
 callq   std::panicking::begin_panic_fmt
 jmp     .LBB7390_9
.LBB7390_1223:
 movb    $1, 117920(%rsp)
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %rcx
 callq   <I as core::iter::traits::collect::IntoIterator>::into_iter
 movq    %rax, %rcx
 movq    %rcx, 600(%rsp)
 jmp     .LBB7390_1224
.LBB7390_1224:
 movq    600(%rsp), %rax
 movq    %rax, 116072(%rsp)
 jmp     .LBB7390_1227
.LBB7390_1225:
 testb   $1, 117920(%rsp)
 jne     .LBB7390_1253
 jmp     .LBB7390_85
.LBB7390_1226:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1225
.LBB7390_1227:
 leaq    116112(%rsp), %rcx
 leaq    116072(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1228
.LBB7390_1228:
 movq    116112(%rsp), %rax
 testq   %rax, %rax
 setne   %al
 movzbl  %al, %eax
 je      .LBB7390_1232
 jmp     .LBB7390_1776
.LBB7390_1776:
 jmp     .LBB7390_1233
.LBB7390_1229:
 testb   $1, 117919(%rsp)
 jne     .LBB7390_1250
 jmp     .LBB7390_1249
.LBB7390_1230:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1229
 ud2
.LBB7390_1232:
 movb    $0, 117919(%rsp)
 leaq    116072(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1251
.LBB7390_1233:
 movups  116112(%rsp), %xmm0
 movups  116128(%rsp), %xmm1
 movaps  %xmm1, 116160(%rsp)
 movaps  %xmm0, 116144(%rsp)
 movaps  116144(%rsp), %xmm0
 movaps  116160(%rsp), %xmm1
 movaps  %xmm1, 116192(%rsp)
 movaps  %xmm0, 116176(%rsp)
 movb    $1, 117919(%rsp)
 movaps  116176(%rsp), %xmm0
 movaps  116192(%rsp), %xmm1
 movaps  %xmm1, 116096(%rsp)
 movaps  %xmm0, 116080(%rsp)
 movb    $0, 117919(%rsp)
 movaps  116080(%rsp), %xmm0
 movaps  116096(%rsp), %xmm1
 movaps  %xmm1, 116224(%rsp)
 movaps  %xmm0, 116208(%rsp)
 movb    $1, 117918(%rsp)
 movaps  116208(%rsp), %xmm0
 movaps  116224(%rsp), %xmm1
 movaps  %xmm1, 116816(%rsp)
 movaps  %xmm0, 116800(%rsp)
 leaq    116840(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1234
.LBB7390_1234:
 movq    2792(%rsp), %rdx
 movb    $0, 117918(%rsp)
 leaq    116536(%rsp), %rcx
 leaq    116800(%rsp), %r8
 leaq    116840(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_1237
.LBB7390_1235:
 testb   $1, 117918(%rsp)
 jne     .LBB7390_1238
 jmp     .LBB7390_1229
.LBB7390_1236:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1235
.LBB7390_1237:
 movb    $0, 117918(%rsp)
 leaq    116272(%rsp), %rcx
 leaq    116536(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1239
.LBB7390_1238:
 leaq    116800(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1229
.LBB7390_1239:
 movq    116272(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1241
 jmp     .LBB7390_1777
.LBB7390_1777:
 jmp     .LBB7390_1242
 ud2
.LBB7390_1241:
 movups  116280(%rsp), %xmm0
 movups  116296(%rsp), %xmm1
 movaps  %xmm1, 117392(%rsp)
 movaps  %xmm0, 117376(%rsp)
 movaps  117376(%rsp), %xmm0
 movaps  117392(%rsp), %xmm1
 movaps  %xmm1, 116256(%rsp)
 movaps  %xmm0, 116240(%rsp)
 leaq    116048(%rsp), %rcx
 leaq    116240(%rsp), %rdx
 callq   alloc::vec::Vec<T,A>::push
 jmp     .LBB7390_1248
.LBB7390_1242:
 leaq    116280(%rsp), %rdx
 leaq    116864(%rsp), %rcx
 movq    %rcx, 576(%rsp)
 movl    $256, %r8d
 movq    %r8, 584(%rsp)
 callq   memcpy
 movq    576(%rsp), %rdx
 movq    584(%rsp), %r8
 leaq    117120(%rsp), %rcx
 movq    %rcx, 592(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    592(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1243
.LBB7390_1243:
 movb    $0, 117919(%rsp)
 leaq    116072(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1246
.LBB7390_1244:
 jmp     .LBB7390_1229
.LBB7390_1245:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1244
.LBB7390_1246:
 leaq    116048(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_1247
.LBB7390_1247:
 movb    $0, 117920(%rsp)
 jmp     .LBB7390_104
.LBB7390_1248:
 movb    $0, 117919(%rsp)
 jmp     .LBB7390_1227
.LBB7390_1249:
 leaq    116072(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1225
.LBB7390_1250:
 leaq    116080(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1249
.LBB7390_1251:
 movb    $0, 117920(%rsp)
 movq    116064(%rsp), %rcx
 movq    %rcx, 117648(%rsp)
 movups  116048(%rsp), %xmm0
 movaps  %xmm0, 117632(%rsp)
 movq    117648(%rsp), %rcx
 movq    %rcx, 117440(%rsp)
 movaps  117632(%rsp), %xmm0
 movups  %xmm0, 117424(%rsp)
 movb    $6, 117416(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    117416(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_1252
.LBB7390_1252:
 movb    $0, 117920(%rsp)
 jmp     .LBB7390_89
.LBB7390_1253:
 leaq    116048(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_85
.LBB7390_1254:
 movb    $99, 102879(%rsp)
 movb    102879(%rsp), %r8b
 leaq    102608(%rsp), %rcx
 leaq    102208(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get
 jmp     .LBB7390_1255
.LBB7390_1255:
 leaq    102344(%rsp), %rcx
 leaq    102608(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1258
.LBB7390_1256:
 leaq    102208(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_85
.LBB7390_1257:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1256
.LBB7390_1258:
 movq    102344(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1260
 jmp     .LBB7390_1778
.LBB7390_1778:
 jmp     .LBB7390_1261
 ud2
.LBB7390_1260:
 movups  102352(%rsp), %xmm0
 movups  102368(%rsp), %xmm1
 movaps  %xmm1, 103408(%rsp)
 movaps  %xmm0, 103392(%rsp)
 movb    $1, 117939(%rsp)
 movaps  103392(%rsp), %xmm0
 movaps  103408(%rsp), %xmm1
 movaps  %xmm1, 102320(%rsp)
 movaps  %xmm0, 102304(%rsp)
 movb    $0, 117939(%rsp)
 movb    $1, 117938(%rsp)
 movaps  102304(%rsp), %xmm0
 movaps  102320(%rsp), %xmm1
 movaps  %xmm1, 104000(%rsp)
 movaps  %xmm0, 103984(%rsp)
 leaq    104024(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1266
.LBB7390_1261:
 leaq    102352(%rsp), %rdx
 leaq    102880(%rsp), %rcx
 movq    %rcx, 552(%rsp)
 movl    $256, %r8d
 movq    %r8, 560(%rsp)
 callq   memcpy
 movq    552(%rsp), %rdx
 movq    560(%rsp), %r8
 leaq    103136(%rsp), %rcx
 movq    %rcx, 568(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    568(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1262
.LBB7390_1262:
 jmp     .LBB7390_1265
.LBB7390_1263:
 jmp     .LBB7390_1256
.LBB7390_1264:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1263
.LBB7390_1265:
 movb    $0, 117939(%rsp)
 leaq    102208(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_104
.LBB7390_1266:
 movq    2792(%rsp), %rdx
 movb    $0, 117938(%rsp)
 leaq    103720(%rsp), %rcx
 leaq    103984(%rsp), %r8
 leaq    104024(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_1269
.LBB7390_1267:
 testb   $1, 117938(%rsp)
 jne     .LBB7390_1271
 jmp     .LBB7390_1270
.LBB7390_1268:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1267
.LBB7390_1269:
 movb    $0, 117938(%rsp)
 leaq    103456(%rsp), %rcx
 leaq    103720(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1272
.LBB7390_1270:
 testb   $1, 117939(%rsp)
 jne     .LBB7390_1351
 jmp     .LBB7390_1256
.LBB7390_1271:
 leaq    103984(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1270
.LBB7390_1272:
 movq    103456(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1275
 jmp     .LBB7390_1779
.LBB7390_1779:
 jmp     .LBB7390_1276
.LBB7390_1273:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1270
 ud2
.LBB7390_1275:
 movups  103464(%rsp), %xmm0
 movups  103480(%rsp), %xmm1
 movaps  %xmm1, 104576(%rsp)
 movaps  %xmm0, 104560(%rsp)
 movb    $1, 117937(%rsp)
 movaps  104560(%rsp), %xmm0
 movaps  104576(%rsp), %xmm1
 movaps  %xmm1, 103440(%rsp)
 movaps  %xmm0, 103424(%rsp)
 leaq    104600(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1281
.LBB7390_1276:
 leaq    103464(%rsp), %rdx
 leaq    104048(%rsp), %rcx
 movq    %rcx, 528(%rsp)
 movl    $256, %r8d
 movq    %r8, 536(%rsp)
 callq   memcpy
 movq    528(%rsp), %rdx
 movq    536(%rsp), %r8
 leaq    104304(%rsp), %rcx
 movq    %rcx, 544(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    544(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1277
.LBB7390_1277:
 jmp     .LBB7390_1280
.LBB7390_1278:
 jmp     .LBB7390_1270
.LBB7390_1279:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1278
.LBB7390_1280:
 movb    $0, 117937(%rsp)
 jmp     .LBB7390_1265
.LBB7390_1281:
 movb    $1, 117936(%rsp)
 movb    $-127, 105215(%rsp)
 movb    105215(%rsp), %r8b
 leaq    104944(%rsp), %rcx
 leaq    102208(%rsp), %rdx
 callq   jodin_rs::ast::IndexedPair::get_all
 jmp     .LBB7390_1284
.LBB7390_1282:
 testb   $1, 117937(%rsp)
 jne     .LBB7390_1350
 jmp     .LBB7390_1270
.LBB7390_1283:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1282
.LBB7390_1284:
 leaq    104680(%rsp), %rcx
 leaq    104944(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1287
.LBB7390_1285:
 testb   $1, 117936(%rsp)
 jne     .LBB7390_1349
 jmp     .LBB7390_1282
.LBB7390_1286:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1285
.LBB7390_1287:
 movq    104680(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1289
 jmp     .LBB7390_1780
.LBB7390_1780:
 jmp     .LBB7390_1290
 ud2
.LBB7390_1289:
 movq    104704(%rsp), %rcx
 movq    %rcx, 105744(%rsp)
 movups  104688(%rsp), %xmm0
 movaps  %xmm0, 105728(%rsp)
 movq    105744(%rsp), %rcx
 movq    %rcx, 104672(%rsp)
 movaps  105728(%rsp), %xmm0
 movaps  %xmm0, 104656(%rsp)
 leaq    104624(%rsp), %rcx
 leaq    104656(%rsp), %rdx
 callq   <alloc::vec::Vec<T,A> as core::iter::traits::collect::IntoIterator>::into_iter
 jmp     .LBB7390_1295
.LBB7390_1290:
 leaq    104688(%rsp), %rdx
 leaq    105216(%rsp), %rcx
 movq    %rcx, 504(%rsp)
 movl    $256, %r8d
 movq    %r8, 512(%rsp)
 callq   memcpy
 movq    504(%rsp), %rdx
 movq    512(%rsp), %r8
 leaq    105472(%rsp), %rcx
 movq    %rcx, 520(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    520(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1291
.LBB7390_1291:
 jmp     .LBB7390_1294
.LBB7390_1292:
 jmp     .LBB7390_1285
.LBB7390_1293:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1292
.LBB7390_1294:
 leaq    104600(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<(jodin_rs::ast::jodin_node::JodinNode,jodin_rs::ast::jodin_node::JodinNode)>>
 jmp     .LBB7390_1338
.LBB7390_1295:
 movq    104624(%rsp), %rax
 movq    %rax, 105760(%rsp)
 movq    104632(%rsp), %rax
 movq    %rax, 105768(%rsp)
 movq    104640(%rsp), %rax
 movq    %rax, 105776(%rsp)
 movq    104648(%rsp), %rax
 movq    %rax, 105784(%rsp)
.LBB7390_1296:
 leaq    105824(%rsp), %rcx
 leaq    105760(%rsp), %rdx
 callq   <alloc::vec::into_iter::IntoIter<T,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1297
.LBB7390_1297:
 movq    105824(%rsp), %rax
 testq   %rax, %rax
 setne   %al
 movzbl  %al, %eax
 je      .LBB7390_1301
 jmp     .LBB7390_1781
.LBB7390_1781:
 jmp     .LBB7390_1302
.LBB7390_1298:
 testb   $1, 117935(%rsp)
 jne     .LBB7390_1346
 jmp     .LBB7390_1345
.LBB7390_1299:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1298
 ud2
.LBB7390_1301:
 movb    $0, 117935(%rsp)
 leaq    105760(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1347
.LBB7390_1302:
 movups  105824(%rsp), %xmm0
 movups  105840(%rsp), %xmm1
 movaps  %xmm1, 105872(%rsp)
 movaps  %xmm0, 105856(%rsp)
 movaps  105856(%rsp), %xmm0
 movaps  105872(%rsp), %xmm1
 movaps  %xmm1, 105904(%rsp)
 movaps  %xmm0, 105888(%rsp)
 movb    $1, 117935(%rsp)
 movaps  105888(%rsp), %xmm0
 movaps  105904(%rsp), %xmm1
 movaps  %xmm1, 105808(%rsp)
 movaps  %xmm0, 105792(%rsp)
 movb    $0, 117935(%rsp)
 movaps  105792(%rsp), %xmm0
 movaps  105808(%rsp), %xmm1
 movaps  %xmm1, 105936(%rsp)
 movaps  %xmm0, 105920(%rsp)
 movaps  105920(%rsp), %xmm0
 movaps  105936(%rsp), %xmm1
 movaps  %xmm1, 106016(%rsp)
 movaps  %xmm0, 106000(%rsp)
 leaq    105960(%rsp), %rcx
 leaq    106000(%rsp), %rdx
 callq   pest::iterators::pair::Pair<R>::into_inner
 jmp     .LBB7390_1303
.LBB7390_1303:
 leaq    106632(%rsp), %rcx
 leaq    105960(%rsp), %rdx
 callq   <pest::iterators::pairs::Pairs<R> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1304
.LBB7390_1304:
 leaq    .L__unnamed_412(%rip), %r8
 leaq    106600(%rsp), %rcx
 leaq    106632(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1307
.LBB7390_1305:
 leaq    105960(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1298
.LBB7390_1306:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1305
.LBB7390_1307:
 movb    $1, 117934(%rsp)
 leaq    106664(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1308
.LBB7390_1308:
 movq    2792(%rsp), %rdx
 movb    $0, 117934(%rsp)
 leaq    106336(%rsp), %rcx
 leaq    106600(%rsp), %r8
 leaq    106664(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_1311
.LBB7390_1309:
 testb   $1, 117934(%rsp)
 jne     .LBB7390_1312
 jmp     .LBB7390_1305
.LBB7390_1310:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1309
.LBB7390_1311:
 movb    $0, 117934(%rsp)
 leaq    106072(%rsp), %rcx
 leaq    106336(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1313
.LBB7390_1312:
 leaq    106600(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1305
.LBB7390_1313:
 movq    106072(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1315
 jmp     .LBB7390_1782
.LBB7390_1782:
 jmp     .LBB7390_1316
 ud2
.LBB7390_1315:
 movups  106080(%rsp), %xmm0
 movups  106096(%rsp), %xmm1
 movaps  %xmm1, 107216(%rsp)
 movaps  %xmm0, 107200(%rsp)
 movb    $1, 117933(%rsp)
 movaps  107200(%rsp), %xmm0
 movaps  107216(%rsp), %xmm1
 movaps  %xmm1, 106048(%rsp)
 movaps  %xmm0, 106032(%rsp)
 leaq    107832(%rsp), %rcx
 leaq    105960(%rsp), %rdx
 callq   <pest::iterators::pairs::Pairs<R> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1321
.LBB7390_1316:
 leaq    106080(%rsp), %rdx
 leaq    106688(%rsp), %rcx
 movq    %rcx, 480(%rsp)
 movl    $256, %r8d
 movq    %r8, 488(%rsp)
 callq   memcpy
 movq    480(%rsp), %rdx
 movq    488(%rsp), %r8
 leaq    106944(%rsp), %rcx
 movq    %rcx, 496(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    496(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1317
.LBB7390_1317:
 jmp     .LBB7390_1320
.LBB7390_1318:
 jmp     .LBB7390_1305
.LBB7390_1319:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1318
.LBB7390_1320:
 movb    $0, 117933(%rsp)
 leaq    105960(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1337
.LBB7390_1321:
 leaq    .L__unnamed_413(%rip), %r8
 leaq    107800(%rsp), %rcx
 leaq    107832(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1324
.LBB7390_1322:
 testb   $1, 117933(%rsp)
 jne     .LBB7390_1343
 jmp     .LBB7390_1305
.LBB7390_1323:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1322
.LBB7390_1324:
 movb    $1, 117932(%rsp)
 leaq    107864(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1325
.LBB7390_1325:
 movq    2792(%rsp), %rdx
 movb    $0, 117932(%rsp)
 leaq    107536(%rsp), %rcx
 leaq    107800(%rsp), %r8
 leaq    107864(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_1328
.LBB7390_1326:
 testb   $1, 117932(%rsp)
 jne     .LBB7390_1329
 jmp     .LBB7390_1322
.LBB7390_1327:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1326
.LBB7390_1328:
 movb    $0, 117932(%rsp)
 leaq    107272(%rsp), %rcx
 leaq    107536(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1330
.LBB7390_1329:
 leaq    107800(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1322
.LBB7390_1330:
 movq    107272(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1332
 jmp     .LBB7390_1783
.LBB7390_1783:
 jmp     .LBB7390_1333
 ud2
.LBB7390_1332:
 movups  107280(%rsp), %xmm0
 movups  107296(%rsp), %xmm1
 movaps  %xmm1, 108416(%rsp)
 movaps  %xmm0, 108400(%rsp)
 movb    $1, 117931(%rsp)
 movaps  108400(%rsp), %xmm0
 movaps  108416(%rsp), %xmm1
 movaps  %xmm1, 107248(%rsp)
 movaps  %xmm0, 107232(%rsp)
 movb    $0, 117933(%rsp)
 movaps  106032(%rsp), %xmm0
 movaps  106048(%rsp), %xmm1
 movaps  %xmm1, 108512(%rsp)
 movaps  %xmm0, 108496(%rsp)
 movb    $0, 117931(%rsp)
 movaps  107232(%rsp), %xmm0
 movaps  107248(%rsp), %xmm1
 movaps  %xmm1, 108544(%rsp)
 movaps  %xmm0, 108528(%rsp)
 movaps  108496(%rsp), %xmm0
 movaps  108512(%rsp), %xmm1
 movaps  %xmm1, 108448(%rsp)
 movaps  %xmm0, 108432(%rsp)
 movaps  108528(%rsp), %xmm0
 movaps  108544(%rsp), %xmm1
 movups  %xmm1, 108480(%rsp)
 movups  %xmm0, 108464(%rsp)
 leaq    104600(%rsp), %rcx
 leaq    108432(%rsp), %rdx
 callq   alloc::vec::Vec<T,A>::push
 jmp     .LBB7390_1339
.LBB7390_1333:
 leaq    107280(%rsp), %rdx
 leaq    107888(%rsp), %rcx
 movq    %rcx, 456(%rsp)
 movl    $256, %r8d
 movq    %r8, 464(%rsp)
 callq   memcpy
 movq    456(%rsp), %rdx
 movq    464(%rsp), %r8
 leaq    108144(%rsp), %rcx
 movq    %rcx, 472(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    472(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1334
.LBB7390_1334:
 movb    $0, 117931(%rsp)
 leaq    106032(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1320
.LBB7390_1335:
 jmp     .LBB7390_1322
.LBB7390_1336:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1335
.LBB7390_1337:
 movb    $0, 117935(%rsp)
 leaq    105760(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1294
.LBB7390_1338:
 movb    $0, 117936(%rsp)
 leaq    103424(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1280
.LBB7390_1339:
 movb    $0, 117931(%rsp)
 movb    $0, 117933(%rsp)
 leaq    105960(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1344
.LBB7390_1340:
 testb   $1, 117931(%rsp)
 jne     .LBB7390_1342
 jmp     .LBB7390_1322
.LBB7390_1341:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1340
.LBB7390_1342:
 leaq    107232(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1322
.LBB7390_1343:
 leaq    106032(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1305
.LBB7390_1344:
 movb    $0, 117935(%rsp)
 jmp     .LBB7390_1296
.LBB7390_1345:
 leaq    105760(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1292
.LBB7390_1346:
 leaq    105792(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1345
.LBB7390_1347:
 movb    $0, 117937(%rsp)
 movaps  103424(%rsp), %xmm0
 movaps  103440(%rsp), %xmm1
 movaps  %xmm1, 108800(%rsp)
 movaps  %xmm0, 108784(%rsp)
 movb    $0, 117936(%rsp)
 movq    104616(%rsp), %rcx
 movq    %rcx, 108832(%rsp)
 movups  104600(%rsp), %xmm0
 movaps  %xmm0, 108816(%rsp)
 movaps  108784(%rsp), %xmm0
 movaps  108800(%rsp), %xmm1
 movups  %xmm1, 108592(%rsp)
 movups  %xmm0, 108576(%rsp)
 movq    108832(%rsp), %rcx
 movq    %rcx, 108624(%rsp)
 movaps  108816(%rsp), %xmm0
 movups  %xmm0, 108608(%rsp)
 movb    $30, 108568(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    108568(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_1348
.LBB7390_1348:
 movb    $0, 117936(%rsp)
 movb    $0, 117937(%rsp)
 movb    $0, 117939(%rsp)
 leaq    102208(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::IndexedPair>
 jmp     .LBB7390_89
.LBB7390_1349:
 leaq    104600(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<(jodin_rs::ast::jodin_node::JodinNode,jodin_rs::ast::jodin_node::JodinNode)>>
 jmp     .LBB7390_1282
.LBB7390_1350:
 leaq    103424(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1270
.LBB7390_1351:
 leaq    102304(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1256
.LBB7390_1352:
 leaq    9296(%rsp), %rcx
 leaq    9560(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1353
.LBB7390_1353:
 movq    9296(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1355
 jmp     .LBB7390_1784
.LBB7390_1784:
 jmp     .LBB7390_1356
 ud2
.LBB7390_1355:
 movq    9304(%rsp), %rax
 movq    %rax, 10336(%rsp)
 movq    9312(%rsp), %rax
 movq    %rax, 10344(%rsp)
 movq    9320(%rsp), %rax
 movq    %rax, 10352(%rsp)
 movq    9328(%rsp), %rax
 movq    %rax, 10360(%rsp)
 movq    10336(%rsp), %rax
 movq    %rax, 3376(%rsp)
 movq    10344(%rsp), %rax
 movq    %rax, 3384(%rsp)
 movq    10352(%rsp), %rax
 movq    %rax, 3392(%rsp)
 movq    10360(%rsp), %rax
 movq    %rax, 3400(%rsp)
 jmp     .LBB7390_89
.LBB7390_1356:
 leaq    9304(%rsp), %rdx
 leaq    9824(%rsp), %rcx
 movq    %rcx, 432(%rsp)
 movl    $256, %r8d
 movq    %r8, 440(%rsp)
 callq   memcpy
 movq    432(%rsp), %rdx
 movq    440(%rsp), %r8
 leaq    10080(%rsp), %rcx
 movq    %rcx, 448(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    448(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1357
.LBB7390_1357:
 jmp     .LBB7390_104
.LBB7390_1358:
 jmp     .LBB7390_85
.LBB7390_1359:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1358
.LBB7390_1360:
 leaq    .L__unnamed_414(%rip), %r8
 leaq    40616(%rsp), %rcx
 leaq    40648(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1363
.LBB7390_1361:
 leaq    40608(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_85
.LBB7390_1362:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1361
.LBB7390_1363:
 movb    $1, 117983(%rsp)
 leaq    40712(%rsp), %rcx
 leaq    40608(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1364
.LBB7390_1364:
 leaq    .L__unnamed_415(%rip), %r8
 leaq    40680(%rsp), %rcx
 leaq    40712(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1367
.LBB7390_1365:
 leaq    40616(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1361
.LBB7390_1366:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1365
.LBB7390_1367:
 movb    $1, 117984(%rsp)
 movups  40680(%rsp), %xmm0
 movups  40696(%rsp), %xmm1
 movaps  %xmm1, 41536(%rsp)
 movaps  %xmm0, 41520(%rsp)
 leaq    41560(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1368
.LBB7390_1368:
 movq    2792(%rsp), %rdx
 movb    $0, 117984(%rsp)
 leaq    41256(%rsp), %rcx
 leaq    41520(%rsp), %r8
 leaq    41560(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_1371
.LBB7390_1369:
 testb   $1, 117984(%rsp)
 jne     .LBB7390_1372
 jmp     .LBB7390_1365
.LBB7390_1370:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1369
.LBB7390_1371:
 movb    $0, 117984(%rsp)
 leaq    40992(%rsp), %rcx
 leaq    41256(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1373
.LBB7390_1372:
 leaq    41520(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1365
.LBB7390_1373:
 movq    40992(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1375
 jmp     .LBB7390_1785
.LBB7390_1785:
 jmp     .LBB7390_1376
 ud2
.LBB7390_1375:
 movq    2792(%rsp), %rdx
 movups  41000(%rsp), %xmm0
 movups  41016(%rsp), %xmm1
 movaps  %xmm1, 42112(%rsp)
 movaps  %xmm0, 42096(%rsp)
 movaps  42096(%rsp), %xmm0
 movaps  42112(%rsp), %xmm1
 movaps  %xmm1, 40976(%rsp)
 movaps  %xmm0, 40960(%rsp)
 movb    $0, 117983(%rsp)
 movups  40616(%rsp), %xmm0
 movups  40632(%rsp), %xmm1
 movaps  %xmm1, 42768(%rsp)
 movaps  %xmm0, 42752(%rsp)
 leaq    42488(%rsp), %rcx
 leaq    42752(%rsp), %r8
 callq   jodin_rs::ast::JodinNodeGenerator::new_intermediate_type
 jmp     .LBB7390_1386
.LBB7390_1376:
 leaq    41000(%rsp), %rdx
 leaq    41584(%rsp), %rcx
 movq    %rcx, 408(%rsp)
 movl    $256, %r8d
 movq    %r8, 416(%rsp)
 callq   memcpy
 movq    408(%rsp), %rdx
 movq    416(%rsp), %r8
 leaq    41840(%rsp), %rcx
 movq    %rcx, 424(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    424(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1377
.LBB7390_1377:
 leaq    40616(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1380
.LBB7390_1378:
 testb   $1, 117983(%rsp)
 jne     .LBB7390_1390
 jmp     .LBB7390_1381
.LBB7390_1379:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1378
.LBB7390_1380:
 movb    $0, 117983(%rsp)
 leaq    40608(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1383
.LBB7390_1381:
 leaq    40608(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1384
.LBB7390_1382:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1381
.LBB7390_1383:
 jmp     .LBB7390_104
.LBB7390_1384:
 jmp     .LBB7390_85
.LBB7390_1385:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1384
.LBB7390_1386:
 leaq    42224(%rsp), %rcx
 leaq    42488(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1389
.LBB7390_1387:
 leaq    40960(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1378
.LBB7390_1388:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1387
.LBB7390_1389:
 movq    42224(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1392
 jmp     .LBB7390_1786
.LBB7390_1786:
 jmp     .LBB7390_1393
.LBB7390_1390:
 leaq    40616(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1381
 ud2
.LBB7390_1392:
 movups  42312(%rsp), %xmm0
 movaps  %xmm0, 43376(%rsp)
 movups  42296(%rsp), %xmm0
 movaps  %xmm0, 43360(%rsp)
 movups  42232(%rsp), %xmm0
 movups  42248(%rsp), %xmm1
 movups  42264(%rsp), %xmm2
 movups  42280(%rsp), %xmm3
 movaps  %xmm3, 43344(%rsp)
 movaps  %xmm2, 43328(%rsp)
 movaps  %xmm1, 43312(%rsp)
 movaps  %xmm0, 43296(%rsp)
 movaps  43376(%rsp), %xmm0
 movaps  %xmm0, 42208(%rsp)
 movaps  43360(%rsp), %xmm0
 movaps  %xmm0, 42192(%rsp)
 movaps  43296(%rsp), %xmm0
 movaps  43312(%rsp), %xmm1
 movaps  43328(%rsp), %xmm2
 movaps  43344(%rsp), %xmm3
 movaps  %xmm3, 42176(%rsp)
 movaps  %xmm2, 42160(%rsp)
 movaps  %xmm1, 42144(%rsp)
 movaps  %xmm0, 42128(%rsp)
 movaps  40960(%rsp), %xmm0
 movaps  40976(%rsp), %xmm1
 movups  %xmm1, 40768(%rsp)
 movups  %xmm0, 40752(%rsp)
 movaps  42208(%rsp), %xmm0
 movups  %xmm0, 40864(%rsp)
 movaps  42192(%rsp), %xmm0
 movups  %xmm0, 40848(%rsp)
 movaps  42128(%rsp), %xmm0
 movaps  42144(%rsp), %xmm1
 movaps  42160(%rsp), %xmm2
 movaps  42176(%rsp), %xmm3
 movups  %xmm3, 40832(%rsp)
 movups  %xmm2, 40816(%rsp)
 movups  %xmm1, 40800(%rsp)
 movups  %xmm0, 40784(%rsp)
 movb    $8, 40744(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    40744(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_1403
.LBB7390_1393:
 leaq    42232(%rsp), %rdx
 leaq    42784(%rsp), %rcx
 movq    %rcx, 384(%rsp)
 movl    $256, %r8d
 movq    %r8, 392(%rsp)
 callq   memcpy
 movq    384(%rsp), %rdx
 movq    392(%rsp), %r8
 leaq    43040(%rsp), %rcx
 movq    %rcx, 400(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    400(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1394
.LBB7390_1394:
 leaq    40960(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1398
.LBB7390_1395:
 leaq    40960(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1397
.LBB7390_1396:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1395
.LBB7390_1397:
 leaq    40608(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1401
.LBB7390_1398:
 movb    $0, 117983(%rsp)
 leaq    40608(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1400
.LBB7390_1399:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1397
.LBB7390_1400:
 jmp     .LBB7390_1383
.LBB7390_1401:
 jmp     .LBB7390_1384
.LBB7390_1402:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1401
.LBB7390_1403:
 movb    $0, 117983(%rsp)
 leaq    40608(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1404
.LBB7390_1404:
 jmp     .LBB7390_89
.LBB7390_1405:
 movb    $1, 117923(%rsp)
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 movq    3296(%rsp), %rcx
 callq   <I as core::iter::traits::collect::IntoIterator>::into_iter
 movq    %rax, %rcx
 movq    %rcx, 376(%rsp)
 jmp     .LBB7390_1406
.LBB7390_1406:
 movq    376(%rsp), %rax
 movq    %rax, 114456(%rsp)
 jmp     .LBB7390_1409
.LBB7390_1407:
 testb   $1, 117923(%rsp)
 jne     .LBB7390_1435
 jmp     .LBB7390_85
.LBB7390_1408:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1407
.LBB7390_1409:
 leaq    114496(%rsp), %rcx
 leaq    114456(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1410
.LBB7390_1410:
 movq    114496(%rsp), %rax
 testq   %rax, %rax
 setne   %al
 movzbl  %al, %eax
 je      .LBB7390_1414
 jmp     .LBB7390_1787
.LBB7390_1787:
 jmp     .LBB7390_1415
.LBB7390_1411:
 testb   $1, 117922(%rsp)
 jne     .LBB7390_1432
 jmp     .LBB7390_1431
.LBB7390_1412:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1411
 ud2
.LBB7390_1414:
 movb    $0, 117922(%rsp)
 leaq    114456(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1433
.LBB7390_1415:
 movups  114496(%rsp), %xmm0
 movups  114512(%rsp), %xmm1
 movaps  %xmm1, 114544(%rsp)
 movaps  %xmm0, 114528(%rsp)
 movaps  114528(%rsp), %xmm0
 movaps  114544(%rsp), %xmm1
 movaps  %xmm1, 114576(%rsp)
 movaps  %xmm0, 114560(%rsp)
 movb    $1, 117922(%rsp)
 movaps  114560(%rsp), %xmm0
 movaps  114576(%rsp), %xmm1
 movaps  %xmm1, 114480(%rsp)
 movaps  %xmm0, 114464(%rsp)
 movb    $0, 117922(%rsp)
 movaps  114464(%rsp), %xmm0
 movaps  114480(%rsp), %xmm1
 movaps  %xmm1, 114608(%rsp)
 movaps  %xmm0, 114592(%rsp)
 movb    $1, 117921(%rsp)
 movaps  114592(%rsp), %xmm0
 movaps  114608(%rsp), %xmm1
 movaps  %xmm1, 115200(%rsp)
 movaps  %xmm0, 115184(%rsp)
 leaq    115224(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1416
.LBB7390_1416:
 movq    2792(%rsp), %rdx
 movb    $0, 117921(%rsp)
 leaq    114920(%rsp), %rcx
 leaq    115184(%rsp), %r8
 leaq    115224(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_1419
.LBB7390_1417:
 testb   $1, 117921(%rsp)
 jne     .LBB7390_1420
 jmp     .LBB7390_1411
.LBB7390_1418:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1417
.LBB7390_1419:
 movb    $0, 117921(%rsp)
 leaq    114656(%rsp), %rcx
 leaq    114920(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1421
.LBB7390_1420:
 leaq    115184(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1411
.LBB7390_1421:
 movq    114656(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1423
 jmp     .LBB7390_1788
.LBB7390_1788:
 jmp     .LBB7390_1424
 ud2
.LBB7390_1423:
 movups  114664(%rsp), %xmm0
 movups  114680(%rsp), %xmm1
 movaps  %xmm1, 115776(%rsp)
 movaps  %xmm0, 115760(%rsp)
 movaps  115760(%rsp), %xmm0
 movaps  115776(%rsp), %xmm1
 movaps  %xmm1, 114640(%rsp)
 movaps  %xmm0, 114624(%rsp)
 leaq    114432(%rsp), %rcx
 leaq    114624(%rsp), %rdx
 callq   alloc::vec::Vec<T,A>::push
 jmp     .LBB7390_1430
.LBB7390_1424:
 leaq    114664(%rsp), %rdx
 leaq    115248(%rsp), %rcx
 movq    %rcx, 352(%rsp)
 movl    $256, %r8d
 movq    %r8, 360(%rsp)
 callq   memcpy
 movq    352(%rsp), %rdx
 movq    360(%rsp), %r8
 leaq    115504(%rsp), %rcx
 movq    %rcx, 368(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    368(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1425
.LBB7390_1425:
 movb    $0, 117922(%rsp)
 leaq    114456(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1428
.LBB7390_1426:
 jmp     .LBB7390_1411
.LBB7390_1427:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1426
.LBB7390_1428:
 leaq    114432(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_1429
.LBB7390_1429:
 movb    $0, 117923(%rsp)
 jmp     .LBB7390_104
.LBB7390_1430:
 movb    $0, 117922(%rsp)
 jmp     .LBB7390_1409
.LBB7390_1431:
 leaq    114456(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_1407
.LBB7390_1432:
 leaq    114464(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1431
.LBB7390_1433:
 movb    $0, 117923(%rsp)
 movq    114448(%rsp), %rcx
 movq    %rcx, 116032(%rsp)
 movups  114432(%rsp), %xmm0
 movaps  %xmm0, 116016(%rsp)
 movq    116032(%rsp), %rcx
 movq    %rcx, 115824(%rsp)
 movaps  116016(%rsp), %xmm0
 movups  %xmm0, 115808(%rsp)
 movb    $20, 115800(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    115800(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_1434
.LBB7390_1434:
 movb    $0, 117923(%rsp)
 jmp     .LBB7390_89
.LBB7390_1435:
 leaq    114432(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_85
.LBB7390_1436:
 cmpq    $1, 2848(%rsp)
 jae     .LBB7390_1471
 jmp     .LBB7390_1470
.LBB7390_1437:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 cmpq    $173, %rax
 jne     .LBB7390_1436
 movq    2840(%rsp), %rax
 movzbl  1(%rax), %eax
 cmpq    $176, %rax
 jne     .LBB7390_1436
 leaq    29736(%rsp), %rcx
 leaq    29512(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1440
.LBB7390_1440:
 leaq    .L__unnamed_416(%rip), %r8
 leaq    29704(%rsp), %rcx
 leaq    29736(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1443
.LBB7390_1441:
 leaq    29512(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_85
.LBB7390_1442:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1441
.LBB7390_1443:
 leaq    29664(%rsp), %rcx
 leaq    29704(%rsp), %rdx
 callq   pest::iterators::pair::Pair<R>::into_inner
 jmp     .LBB7390_1444
.LBB7390_1444:
 leaq    29632(%rsp), %rcx
 leaq    29664(%rsp), %rdx
 callq   <pest::iterators::pairs::Pairs<R> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1445
.LBB7390_1445:
 leaq    .L__unnamed_417(%rip), %r8
 leaq    29600(%rsp), %rcx
 leaq    29632(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1448
.LBB7390_1446:
 leaq    29664(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1441
.LBB7390_1447:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1446
.LBB7390_1448:
 leaq    29600(%rsp), %rcx
 callq   pest::iterators::pair::Pair<R>::as_rule
 movb    %al, %cl
 movb    %cl, 351(%rsp)
 jmp     .LBB7390_1449
.LBB7390_1449:
 movb    351(%rsp), %cl
 movb    %cl, 118196(%rsp)
 leaq    29600(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1452
.LBB7390_1450:
 leaq    29600(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1446
.LBB7390_1451:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1450
.LBB7390_1452:
 leaq    29664(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1453
.LBB7390_1453:
 leaq    30360(%rsp), %rcx
 leaq    29512(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1454
.LBB7390_1454:
 leaq    .L__unnamed_418(%rip), %r8
 leaq    30328(%rsp), %rcx
 leaq    30360(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1455
.LBB7390_1455:
 movb    $1, 117995(%rsp)
 leaq    30392(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1456
.LBB7390_1456:
 movq    2792(%rsp), %rdx
 movb    $0, 117995(%rsp)
 leaq    30064(%rsp), %rcx
 leaq    30328(%rsp), %r8
 leaq    30392(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_1459
.LBB7390_1457:
 testb   $1, 117995(%rsp)
 jne     .LBB7390_1460
 jmp     .LBB7390_1441
.LBB7390_1458:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1457
.LBB7390_1459:
 movb    $0, 117995(%rsp)
 leaq    29800(%rsp), %rcx
 leaq    30064(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1461
.LBB7390_1460:
 leaq    30328(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1441
.LBB7390_1461:
 movq    29800(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1463
 jmp     .LBB7390_1791
.LBB7390_1791:
 jmp     .LBB7390_1464
 ud2
.LBB7390_1463:
 movb    351(%rsp), %al
 movq    29808(%rsp), %rcx
 movq    %rcx, 30928(%rsp)
 movq    29816(%rsp), %rcx
 movq    %rcx, 30936(%rsp)
 movq    29824(%rsp), %rcx
 movq    %rcx, 30944(%rsp)
 movq    29832(%rsp), %rcx
 movq    %rcx, 30952(%rsp)
 movq    30928(%rsp), %rcx
 movq    %rcx, 29768(%rsp)
 movq    30936(%rsp), %rcx
 movq    %rcx, 29776(%rsp)
 movq    30944(%rsp), %rcx
 movq    %rcx, 29784(%rsp)
 movq    30952(%rsp), %rcx
 movq    %rcx, 29792(%rsp)
 movb    %al, 30967(%rsp)
 movq    29768(%rsp), %rax
 movq    %rax, 31000(%rsp)
 movq    29776(%rsp), %rax
 movq    %rax, 31008(%rsp)
 movq    29784(%rsp), %rax
 movq    %rax, 31016(%rsp)
 movq    29792(%rsp), %rax
 movq    %rax, 31024(%rsp)
 movq    31000(%rsp), %rax
 movq    %rax, 30968(%rsp)
 movq    31008(%rsp), %rax
 movq    %rax, 30976(%rsp)
 movq    31016(%rsp), %rax
 movq    %rax, 30984(%rsp)
 movq    31024(%rsp), %rax
 movq    %rax, 30992(%rsp)
 movb    30967(%rsp), %al
 movb    %al, 29560(%rsp)
 movq    30968(%rsp), %rax
 movq    %rax, 29568(%rsp)
 movq    30976(%rsp), %rax
 movq    %rax, 29576(%rsp)
 movq    30984(%rsp), %rax
 movq    %rax, 29584(%rsp)
 movq    30992(%rsp), %rax
 movq    %rax, 29592(%rsp)
 jmp     .LBB7390_1469
.LBB7390_1464:
 leaq    29808(%rsp), %rdx
 leaq    30416(%rsp), %rcx
 movq    %rcx, 320(%rsp)
 movl    $256, %r8d
 movq    %r8, 328(%rsp)
 callq   memcpy
 movq    320(%rsp), %rdx
 movq    328(%rsp), %r8
 leaq    30672(%rsp), %rcx
 movq    %rcx, 336(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    336(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1465
.LBB7390_1465:
 jmp     .LBB7390_1468
.LBB7390_1466:
 jmp     .LBB7390_1441
.LBB7390_1467:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1466
.LBB7390_1468:
 leaq    29512(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_104
.LBB7390_1469:
 movb    29560(%rsp), %cl
 movb    %cl, 319(%rsp)
 movb    %cl, 118198(%rsp)
 movb    $1, 117916(%rsp)
 movups  29568(%rsp), %xmm0
 movups  29584(%rsp), %xmm1
 movaps  %xmm1, 29536(%rsp)
 movaps  %xmm0, 29520(%rsp)
 leaq    33184(%rsp), %rcx
 leaq    29512(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1500
.LBB7390_1470:
 movq    $0, 32496(%rsp)
 movb    $-73, 29560(%rsp)
 movq    32496(%rsp), %rax
 movq    %rax, 29568(%rsp)
 movq    32504(%rsp), %rax
 movq    %rax, 29576(%rsp)
 movq    32512(%rsp), %rax
 movq    %rax, 29584(%rsp)
 movq    32520(%rsp), %rax
 movq    %rax, 29592(%rsp)
 jmp     .LBB7390_1469
.LBB7390_1471:
 movq    2840(%rsp), %rax
 movzbl  (%rax), %eax
 movq    %rax, 304(%rsp)
 subq    $173, %rax
 je      .LBB7390_1472
 jmp     .LBB7390_1789
.LBB7390_1789:
 movq    304(%rsp), %rax
 subq    $176, %rax
 je      .LBB7390_1473
 jmp     .LBB7390_1470
.LBB7390_1472:
 leaq    31168(%rsp), %rcx
 leaq    29512(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1488
.LBB7390_1473:
 leaq    31832(%rsp), %rcx
 leaq    29512(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1474
.LBB7390_1474:
 leaq    .L__unnamed_419(%rip), %r8
 leaq    31800(%rsp), %rcx
 leaq    31832(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1475
.LBB7390_1475:
 movb    $1, 117994(%rsp)
 leaq    31864(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1476
.LBB7390_1476:
 movq    2792(%rsp), %rdx
 movb    $0, 117994(%rsp)
 leaq    31536(%rsp), %rcx
 leaq    31800(%rsp), %r8
 leaq    31864(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_1479
.LBB7390_1477:
 testb   $1, 117994(%rsp)
 jne     .LBB7390_1480
 jmp     .LBB7390_1441
.LBB7390_1478:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1477
.LBB7390_1479:
 movb    $0, 117994(%rsp)
 leaq    31272(%rsp), %rcx
 leaq    31536(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1481
.LBB7390_1480:
 leaq    31800(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1441
.LBB7390_1481:
 movq    31272(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1483
 jmp     .LBB7390_1790
.LBB7390_1790:
 jmp     .LBB7390_1484
 ud2
.LBB7390_1483:
 movq    31280(%rsp), %rax
 movq    %rax, 32400(%rsp)
 movq    31288(%rsp), %rax
 movq    %rax, 32408(%rsp)
 movq    31296(%rsp), %rax
 movq    %rax, 32416(%rsp)
 movq    31304(%rsp), %rax
 movq    %rax, 32424(%rsp)
 movq    32400(%rsp), %rax
 movq    %rax, 31240(%rsp)
 movq    32408(%rsp), %rax
 movq    %rax, 31248(%rsp)
 movq    32416(%rsp), %rax
 movq    %rax, 31256(%rsp)
 movq    32424(%rsp), %rax
 movq    %rax, 31264(%rsp)
 movq    31240(%rsp), %rax
 movq    %rax, 32464(%rsp)
 movq    31248(%rsp), %rax
 movq    %rax, 32472(%rsp)
 movq    31256(%rsp), %rax
 movq    %rax, 32480(%rsp)
 movq    31264(%rsp), %rax
 movq    %rax, 32488(%rsp)
 movq    32464(%rsp), %rax
 movq    %rax, 32432(%rsp)
 movq    32472(%rsp), %rax
 movq    %rax, 32440(%rsp)
 movq    32480(%rsp), %rax
 movq    %rax, 32448(%rsp)
 movq    32488(%rsp), %rax
 movq    %rax, 32456(%rsp)
 movb    $-73, 29560(%rsp)
 movq    32432(%rsp), %rax
 movq    %rax, 29568(%rsp)
 movq    32440(%rsp), %rax
 movq    %rax, 29576(%rsp)
 movq    32448(%rsp), %rax
 movq    %rax, 29584(%rsp)
 movq    32456(%rsp), %rax
 movq    %rax, 29592(%rsp)
 jmp     .LBB7390_1469
.LBB7390_1484:
 leaq    31280(%rsp), %rdx
 leaq    31888(%rsp), %rcx
 movq    %rcx, 280(%rsp)
 movl    $256, %r8d
 movq    %r8, 288(%rsp)
 callq   memcpy
 movq    280(%rsp), %rdx
 movq    288(%rsp), %r8
 leaq    32144(%rsp), %rcx
 movq    %rcx, 296(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    296(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1485
.LBB7390_1485:
 jmp     .LBB7390_1468
.LBB7390_1486:
 jmp     .LBB7390_1441
.LBB7390_1487:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1486
.LBB7390_1488:
 leaq    .L__unnamed_420(%rip), %r8
 leaq    31136(%rsp), %rcx
 leaq    31168(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1489
.LBB7390_1489:
 leaq    31096(%rsp), %rcx
 leaq    31136(%rsp), %rdx
 callq   pest::iterators::pair::Pair<R>::into_inner
 jmp     .LBB7390_1490
.LBB7390_1490:
 leaq    31064(%rsp), %rcx
 leaq    31096(%rsp), %rdx
 callq   <pest::iterators::pairs::Pairs<R> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1491
.LBB7390_1491:
 leaq    .L__unnamed_421(%rip), %r8
 leaq    31032(%rsp), %rcx
 leaq    31064(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1494
.LBB7390_1492:
 leaq    31096(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1441
.LBB7390_1493:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1492
.LBB7390_1494:
 leaq    31032(%rsp), %rcx
 callq   pest::iterators::pair::Pair<R>::as_rule
 movb    %al, %cl
 movb    %cl, 279(%rsp)
 jmp     .LBB7390_1495
.LBB7390_1495:
 movb    279(%rsp), %cl
 movb    %cl, 118197(%rsp)
 leaq    31032(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1498
.LBB7390_1496:
 leaq    31032(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1492
.LBB7390_1497:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1496
.LBB7390_1498:
 leaq    31096(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1499
.LBB7390_1499:
 movb    279(%rsp), %al
 movb    %al, 31207(%rsp)
 movq    $0, 31208(%rsp)
 movb    31207(%rsp), %al
 movb    %al, 29560(%rsp)
 movq    31208(%rsp), %rax
 movq    %rax, 29568(%rsp)
 movq    31216(%rsp), %rax
 movq    %rax, 29576(%rsp)
 movq    31224(%rsp), %rax
 movq    %rax, 29584(%rsp)
 movq    31232(%rsp), %rax
 movq    %rax, 29592(%rsp)
 jmp     .LBB7390_1469
.LBB7390_1500:
 leaq    .L__unnamed_422(%rip), %r8
 leaq    33152(%rsp), %rcx
 leaq    33184(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1503
.LBB7390_1501:
 movl    $1, %eax
 xorl    %ecx, %ecx
 cmpq    $0, 29520(%rsp)
 cmoveq  %rcx, %rax
 cmpq    $1, %rax
 je      .LBB7390_1618
 jmp     .LBB7390_1441
.LBB7390_1502:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1501
.LBB7390_1503:
 movq    2792(%rsp), %rdx
 leaq    32888(%rsp), %rcx
 leaq    33152(%rsp), %r8
 callq   jodin_rs::ast::JodinNodeGenerator::new_intermediate_type
 jmp     .LBB7390_1504
.LBB7390_1504:
 leaq    32624(%rsp), %rcx
 leaq    32888(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1505
.LBB7390_1505:
 movq    32624(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1507
 jmp     .LBB7390_1792
.LBB7390_1792:
 jmp     .LBB7390_1508
 ud2
.LBB7390_1507:
 movups  32712(%rsp), %xmm0
 movaps  %xmm0, 33808(%rsp)
 movups  32696(%rsp), %xmm0
 movaps  %xmm0, 33792(%rsp)
 movups  32632(%rsp), %xmm0
 movups  32648(%rsp), %xmm1
 movups  32664(%rsp), %xmm2
 movups  32680(%rsp), %xmm3
 movaps  %xmm3, 33776(%rsp)
 movaps  %xmm2, 33760(%rsp)
 movaps  %xmm1, 33744(%rsp)
 movaps  %xmm0, 33728(%rsp)
 movb    $1, 117993(%rsp)
 movaps  33808(%rsp), %xmm0
 movaps  %xmm0, 32608(%rsp)
 movaps  33792(%rsp), %xmm0
 movaps  %xmm0, 32592(%rsp)
 movaps  33728(%rsp), %xmm0
 movaps  33744(%rsp), %xmm1
 movaps  33760(%rsp), %xmm2
 movaps  33776(%rsp), %xmm3
 movaps  %xmm3, 32576(%rsp)
 movaps  %xmm2, 32560(%rsp)
 movaps  %xmm1, 32544(%rsp)
 movaps  %xmm0, 32528(%rsp)
 leaq    34424(%rsp), %rcx
 leaq    29512(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1513
.LBB7390_1508:
 leaq    32632(%rsp), %rdx
 leaq    33216(%rsp), %rcx
 movq    %rcx, 248(%rsp)
 movl    $256, %r8d
 movq    %r8, 256(%rsp)
 callq   memcpy
 movq    248(%rsp), %rdx
 movq    256(%rsp), %r8
 leaq    33472(%rsp), %rcx
 movq    %rcx, 264(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    264(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1509
.LBB7390_1509:
 jmp     .LBB7390_1512
.LBB7390_1510:
 jmp     .LBB7390_1501
.LBB7390_1511:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1510
.LBB7390_1512:
 movb    $0, 117993(%rsp)
 movl    $1, %eax
 xorl    %ecx, %ecx
 cmpq    $0, 29520(%rsp)
 cmoveq  %rcx, %rax
 cmpq    $1, %rax
 je      .LBB7390_1606
 jmp     .LBB7390_1607
.LBB7390_1513:
 leaq    .L__unnamed_423(%rip), %r8
 leaq    34392(%rsp), %rcx
 leaq    34424(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1516
.LBB7390_1514:
 testb   $1, 117993(%rsp)
 jne     .LBB7390_1617
 jmp     .LBB7390_1501
.LBB7390_1515:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1514
.LBB7390_1516:
 movb    $1, 117992(%rsp)
 leaq    34456(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1517
.LBB7390_1517:
 movq    2792(%rsp), %rdx
 movb    $0, 117992(%rsp)
 leaq    34128(%rsp), %rcx
 leaq    34392(%rsp), %r8
 leaq    34456(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_1520
.LBB7390_1518:
 testb   $1, 117992(%rsp)
 jne     .LBB7390_1521
 jmp     .LBB7390_1514
.LBB7390_1519:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1518
.LBB7390_1520:
 movb    $0, 117992(%rsp)
 leaq    33864(%rsp), %rcx
 leaq    34128(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1522
.LBB7390_1521:
 leaq    34392(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1514
.LBB7390_1522:
 movq    33864(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1524
 jmp     .LBB7390_1793
.LBB7390_1793:
 jmp     .LBB7390_1525
 ud2
.LBB7390_1524:
 movb    319(%rsp), %dl
 movups  33872(%rsp), %xmm0
 movups  33888(%rsp), %xmm1
 movaps  %xmm1, 35008(%rsp)
 movaps  %xmm0, 34992(%rsp)
 movb    $1, 117991(%rsp)
 movaps  34992(%rsp), %xmm0
 movaps  35008(%rsp), %xmm1
 movaps  %xmm1, 33840(%rsp)
 movaps  %xmm0, 33824(%rsp)
 leaq    35800(%rsp), %rcx
 callq   <jodin_rs::core::privacy::Visibility as core::convert::TryFrom<core::option::Option<jodin_rs::parsing::Rule>>>::try_from
 jmp     .LBB7390_1530
.LBB7390_1525:
 leaq    33872(%rsp), %rdx
 leaq    34480(%rsp), %rcx
 movq    %rcx, 224(%rsp)
 movl    $256, %r8d
 movq    %r8, 232(%rsp)
 callq   memcpy
 movq    224(%rsp), %rdx
 movq    232(%rsp), %r8
 leaq    34736(%rsp), %rcx
 movq    %rcx, 240(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    240(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1526
.LBB7390_1526:
 jmp     .LBB7390_1529
.LBB7390_1527:
 jmp     .LBB7390_1514
.LBB7390_1528:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1527
.LBB7390_1529:
 movb    $0, 117991(%rsp)
 leaq    32528(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::intermediate_type::IntermediateType>
 jmp     .LBB7390_1512
.LBB7390_1530:
 leaq    35536(%rsp), %rcx
 leaq    35800(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1533
.LBB7390_1531:
 testb   $1, 117991(%rsp)
 jne     .LBB7390_1616
 jmp     .LBB7390_1514
.LBB7390_1532:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1531
.LBB7390_1533:
 movzbl  35536(%rsp), %eax
 testb   $1, %al
 je      .LBB7390_1535
 jmp     .LBB7390_1794
.LBB7390_1794:
 jmp     .LBB7390_1536
 ud2
.LBB7390_1535:
 movb    35537(%rsp), %cl
 movb    %cl, 118199(%rsp)
 callq   jodin_rs::core::privacy::VisibilityTag::new
 movb    %al, %cl
 movb    %cl, 223(%rsp)
 jmp     .LBB7390_1541
.LBB7390_1536:
 leaq    35544(%rsp), %rdx
 leaq    36064(%rsp), %rcx
 movq    %rcx, 192(%rsp)
 movl    $256, %r8d
 movq    %r8, 200(%rsp)
 callq   memcpy
 movq    192(%rsp), %rdx
 movq    200(%rsp), %r8
 leaq    36320(%rsp), %rcx
 movq    %rcx, 208(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    208(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1537
.LBB7390_1537:
 jmp     .LBB7390_1540
.LBB7390_1538:
 jmp     .LBB7390_1531
.LBB7390_1539:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1538
.LBB7390_1540:
 jmp     .LBB7390_1550
.LBB7390_1541:
 movb    223(%rsp), %r8b
 leaq    35280(%rsp), %rcx
 leaq    33824(%rsp), %rdx
 callq   jodin_rs::ast::jodin_node::JodinNode::add_tag
 jmp     .LBB7390_1542
.LBB7390_1542:
 leaq    35024(%rsp), %rcx
 leaq    35280(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1543
.LBB7390_1543:
 movb    35024(%rsp), %al
 addb    $-23, %al
 setne   %al
 movzbl  %al, %eax
 je      .LBB7390_1545
 jmp     .LBB7390_1795
.LBB7390_1795:
 jmp     .LBB7390_1546
 ud2
.LBB7390_1545:
 leaq    37688(%rsp), %rcx
 leaq    29512(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1551
.LBB7390_1546:
 leaq    36576(%rsp), %rcx
 movq    %rcx, 168(%rsp)
 leaq    35024(%rsp), %rdx
 movl    $256, %r8d
 movq    %r8, 176(%rsp)
 callq   memcpy
 movq    168(%rsp), %rdx
 movq    176(%rsp), %r8
 leaq    36832(%rsp), %rcx
 movq    %rcx, 184(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    184(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1547
.LBB7390_1547:
 jmp     .LBB7390_1540
.LBB7390_1548:
 jmp     .LBB7390_1538
.LBB7390_1549:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1548
.LBB7390_1550:
 leaq    33824(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1529
.LBB7390_1551:
 leaq    .L__unnamed_424(%rip), %r8
 leaq    37656(%rsp), %rcx
 leaq    37688(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1552
.LBB7390_1552:
 movb    $1, 117990(%rsp)
 leaq    37720(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1553
.LBB7390_1553:
 movq    2792(%rsp), %rdx
 movb    $0, 117990(%rsp)
 leaq    37392(%rsp), %rcx
 leaq    37656(%rsp), %r8
 leaq    37720(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_1556
.LBB7390_1554:
 testb   $1, 117990(%rsp)
 jne     .LBB7390_1557
 jmp     .LBB7390_1531
.LBB7390_1555:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1554
.LBB7390_1556:
 movb    $0, 117990(%rsp)
 leaq    37128(%rsp), %rcx
 leaq    37392(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1558
.LBB7390_1557:
 leaq    37656(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1531
.LBB7390_1558:
 movq    37128(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1560
 jmp     .LBB7390_1796
.LBB7390_1796:
 jmp     .LBB7390_1561
 ud2
.LBB7390_1560:
 movups  37136(%rsp), %xmm0
 movups  37152(%rsp), %xmm1
 movaps  %xmm1, 38272(%rsp)
 movaps  %xmm0, 38256(%rsp)
 movb    $1, 117989(%rsp)
 movaps  38256(%rsp), %xmm0
 movaps  38272(%rsp), %xmm1
 movaps  %xmm1, 37104(%rsp)
 movaps  %xmm0, 37088(%rsp)
 movb    $0, 117989(%rsp)
 movaps  37088(%rsp), %xmm0
 movaps  37104(%rsp), %xmm1
 movaps  %xmm1, 38544(%rsp)
 movaps  %xmm0, 38528(%rsp)
 leaq    38312(%rsp), %rcx
 leaq    38528(%rsp), %rdx
 callq   jodin_rs::ast::jodin_node::JodinNode::into_inner
 jmp     .LBB7390_1566
.LBB7390_1561:
 leaq    37136(%rsp), %rdx
 leaq    37744(%rsp), %rcx
 movq    %rcx, 144(%rsp)
 movl    $256, %r8d
 movq    %r8, 152(%rsp)
 callq   memcpy
 movq    144(%rsp), %rdx
 movq    152(%rsp), %r8
 leaq    38000(%rsp), %rcx
 movq    %rcx, 160(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    160(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1562
.LBB7390_1562:
 jmp     .LBB7390_1565
.LBB7390_1563:
 jmp     .LBB7390_1531
.LBB7390_1564:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1563
.LBB7390_1565:
 movb    $0, 117989(%rsp)
 jmp     .LBB7390_1550
.LBB7390_1566:
 movzbl  38312(%rsp), %eax
 cmpq    $20, %rax
 je      .LBB7390_1569
 jmp     .LBB7390_1570
.LBB7390_1567:
 testb   $1, 117989(%rsp)
 jne     .LBB7390_1615
 jmp     .LBB7390_1531
.LBB7390_1568:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1567
.LBB7390_1569:
 movq    38320(%rsp), %rax
 movq    %rax, 38560(%rsp)
 movq    38328(%rsp), %rax
 movq    %rax, 38568(%rsp)
 movq    38336(%rsp), %rax
 movq    %rax, 38576(%rsp)
 movb    $1, 117988(%rsp)
 movq    38560(%rsp), %rax
 movq    %rax, 38288(%rsp)
 movq    38568(%rsp), %rax
 movq    %rax, 38296(%rsp)
 movq    38576(%rsp), %rax
 movq    %rax, 38304(%rsp)
 movzbl  38312(%rsp), %eax
 cmpq    $20, %rax
 je      .LBB7390_1573
 jmp     .LBB7390_1574
.LBB7390_1570:
 leaq    .L__unnamed_372(%rip), %rcx
 leaq    .L__unnamed_425(%rip), %r8
 movl    $34, %edx
 callq   std::panicking::begin_panic
 jmp     .LBB7390_9
.LBB7390_1571:
 leaq    38312(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_1567
.LBB7390_1572:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1571
.LBB7390_1573:
 movq    29520(%rsp), %rax
 testq   %rax, %rax
 setne   %al
 movzbl  %al, %eax
 je      .LBB7390_1578
 jmp     .LBB7390_1797
.LBB7390_1797:
 jmp     .LBB7390_1579
.LBB7390_1574:
 leaq    38312(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_1573
.LBB7390_1575:
 testb   $1, 117988(%rsp)
 jne     .LBB7390_1614
 jmp     .LBB7390_1567
.LBB7390_1576:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1575
 ud2
.LBB7390_1578:
 leaq    38584(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1587
.LBB7390_1579:
 movb    $0, 117916(%rsp)
 movaps  29520(%rsp), %xmm0
 movaps  29536(%rsp), %xmm1
 movaps  %xmm1, 38624(%rsp)
 movaps  %xmm0, 38608(%rsp)
 movaps  38608(%rsp), %xmm0
 movaps  38624(%rsp), %xmm1
 movaps  %xmm1, 38880(%rsp)
 movaps  %xmm0, 38864(%rsp)
 leaq    38648(%rsp), %rcx
 leaq    38864(%rsp), %rdx
 callq   jodin_rs::ast::jodin_node::JodinNode::into_inner
 jmp     .LBB7390_1580
.LBB7390_1580:
 movzbl  38648(%rsp), %eax
 cmpq    $20, %rax
 jne     .LBB7390_1582
 movq    38656(%rsp), %rax
 movq    %rax, 38904(%rsp)
 movq    38664(%rsp), %rax
 movq    %rax, 38912(%rsp)
 movq    38672(%rsp), %rax
 movq    %rax, 38920(%rsp)
 movb    $1, 117987(%rsp)
 movq    38904(%rsp), %rax
 movq    %rax, 38584(%rsp)
 movq    38912(%rsp), %rax
 movq    %rax, 38592(%rsp)
 movq    38920(%rsp), %rax
 movq    %rax, 38600(%rsp)
 movzbl  38648(%rsp), %eax
 cmpq    $20, %rax
 je      .LBB7390_1585
 jmp     .LBB7390_1586
.LBB7390_1582:
 leaq    .L__unnamed_372(%rip), %rcx
 leaq    .L__unnamed_426(%rip), %r8
 movl    $34, %edx
 callq   std::panicking::begin_panic
 jmp     .LBB7390_9
.LBB7390_1583:
 leaq    38648(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_1575
.LBB7390_1584:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1583
.LBB7390_1585:
 leaq    39528(%rsp), %rcx
 leaq    29512(%rsp), %rdx
 callq   <alloc::boxed::Box<I,A> as core::iter::traits::iterator::Iterator>::next
 jmp     .LBB7390_1588
.LBB7390_1586:
 leaq    38648(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::node_type::JodinNodeType>
 jmp     .LBB7390_1585
.LBB7390_1587:
 movb    $1, 117987(%rsp)
 jmp     .LBB7390_1585
.LBB7390_1588:
 leaq    .L__unnamed_427(%rip), %r8
 leaq    39496(%rsp), %rcx
 leaq    39528(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1591
.LBB7390_1589:
 testb   $1, 117987(%rsp)
 jne     .LBB7390_1613
 jmp     .LBB7390_1575
.LBB7390_1590:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1589
.LBB7390_1591:
 movb    $1, 117986(%rsp)
 leaq    39560(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1592
.LBB7390_1592:
 movq    2792(%rsp), %rdx
 movb    $0, 117986(%rsp)
 leaq    39232(%rsp), %rcx
 leaq    39496(%rsp), %r8
 leaq    39560(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_1595
.LBB7390_1593:
 testb   $1, 117986(%rsp)
 jne     .LBB7390_1596
 jmp     .LBB7390_1589
.LBB7390_1594:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1593
.LBB7390_1595:
 movb    $0, 117986(%rsp)
 leaq    38968(%rsp), %rcx
 leaq    39232(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1597
.LBB7390_1596:
 leaq    39496(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1589
.LBB7390_1597:
 movq    38968(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1599
 jmp     .LBB7390_1798
.LBB7390_1798:
 jmp     .LBB7390_1600
 ud2
.LBB7390_1599:
 movups  38976(%rsp), %xmm0
 movups  38992(%rsp), %xmm1
 movaps  %xmm1, 40112(%rsp)
 movaps  %xmm0, 40096(%rsp)
 movb    $1, 117985(%rsp)
 movaps  40096(%rsp), %xmm0
 movaps  40112(%rsp), %xmm1
 movaps  %xmm1, 38944(%rsp)
 movaps  %xmm0, 38928(%rsp)
 movb    $0, 117991(%rsp)
 movaps  33824(%rsp), %xmm0
 movaps  33840(%rsp), %xmm1
 movaps  %xmm1, 40400(%rsp)
 movaps  %xmm0, 40384(%rsp)
 movb    $0, 117993(%rsp)
 movaps  32608(%rsp), %xmm0
 movaps  %xmm0, 40496(%rsp)
 movaps  32592(%rsp), %xmm0
 movaps  %xmm0, 40480(%rsp)
 movaps  32528(%rsp), %xmm0
 movaps  32544(%rsp), %xmm1
 movaps  32560(%rsp), %xmm2
 movaps  32576(%rsp), %xmm3
 movaps  %xmm3, 40464(%rsp)
 movaps  %xmm2, 40448(%rsp)
 movaps  %xmm1, 40432(%rsp)
 movaps  %xmm0, 40416(%rsp)
 movb    $0, 117988(%rsp)
 movq    38304(%rsp), %rcx
 movq    %rcx, 40528(%rsp)
 movups  38288(%rsp), %xmm0
 movaps  %xmm0, 40512(%rsp)
 movb    $0, 117987(%rsp)
 movq    38600(%rsp), %rcx
 movq    %rcx, 40560(%rsp)
 movups  38584(%rsp), %xmm0
 movaps  %xmm0, 40544(%rsp)
 movb    $0, 117985(%rsp)
 movaps  38928(%rsp), %xmm0
 movaps  38944(%rsp), %xmm1
 movaps  %xmm1, 40592(%rsp)
 movaps  %xmm0, 40576(%rsp)
 movaps  40384(%rsp), %xmm0
 movaps  40400(%rsp), %xmm1
 movups  %xmm1, 40192(%rsp)
 movups  %xmm0, 40176(%rsp)
 movaps  40496(%rsp), %xmm0
 movups  %xmm0, 40288(%rsp)
 movaps  40480(%rsp), %xmm0
 movups  %xmm0, 40272(%rsp)
 movaps  40416(%rsp), %xmm0
 movaps  40432(%rsp), %xmm1
 movaps  40448(%rsp), %xmm2
 movaps  40464(%rsp), %xmm3
 movups  %xmm3, 40256(%rsp)
 movups  %xmm2, 40240(%rsp)
 movups  %xmm1, 40224(%rsp)
 movups  %xmm0, 40208(%rsp)
 movq    40528(%rsp), %rcx
 movq    %rcx, 40320(%rsp)
 movaps  40512(%rsp), %xmm0
 movups  %xmm0, 40304(%rsp)
 movq    40560(%rsp), %rcx
 movq    %rcx, 40344(%rsp)
 movaps  40544(%rsp), %xmm0
 movups  %xmm0, 40328(%rsp)
 movaps  40576(%rsp), %xmm0
 movaps  40592(%rsp), %xmm1
 movups  %xmm1, 40368(%rsp)
 movups  %xmm0, 40352(%rsp)
 movb    $4, 40168(%rsp)
 leaq    40136(%rsp), %rcx
 leaq    40168(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_1609
.LBB7390_1600:
 leaq    38976(%rsp), %rdx
 leaq    39584(%rsp), %rcx
 movq    %rcx, 120(%rsp)
 movl    $256, %r8d
 movq    %r8, 128(%rsp)
 callq   memcpy
 movq    120(%rsp), %rdx
 movq    128(%rsp), %r8
 leaq    39840(%rsp), %rcx
 movq    %rcx, 136(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    136(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1601
.LBB7390_1601:
 movb    $0, 117985(%rsp)
 leaq    38584(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_1604
.LBB7390_1602:
 jmp     .LBB7390_1589
.LBB7390_1603:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1602
.LBB7390_1604:
 movb    $0, 117987(%rsp)
 leaq    38288(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_1605
.LBB7390_1605:
 movb    $0, 117988(%rsp)
 jmp     .LBB7390_1565
.LBB7390_1606:
 testb   $1, 117916(%rsp)
 jne     .LBB7390_1608
.LBB7390_1607:
 movb    $0, 117916(%rsp)
 jmp     .LBB7390_1468
.LBB7390_1608:
 leaq    29520(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1607
.LBB7390_1609:
 movups  40136(%rsp), %xmm0
 movups  40152(%rsp), %xmm1
 movaps  %xmm1, 3392(%rsp)
 movaps  %xmm0, 3376(%rsp)
 movb    $0, 117985(%rsp)
 movb    $0, 117987(%rsp)
 movb    $0, 117988(%rsp)
 movb    $0, 117989(%rsp)
 movb    $0, 117991(%rsp)
 movb    $0, 117993(%rsp)
 movb    $0, 117916(%rsp)
 leaq    29512(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>>
 jmp     .LBB7390_89
.LBB7390_1610:
 testb   $1, 117985(%rsp)
 jne     .LBB7390_1612
 jmp     .LBB7390_1589
.LBB7390_1611:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1610
.LBB7390_1612:
 leaq    38928(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1589
.LBB7390_1613:
 leaq    38584(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_1575
.LBB7390_1614:
 leaq    38288(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_1567
.LBB7390_1615:
 leaq    37088(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1531
.LBB7390_1616:
 leaq    33824(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1514
.LBB7390_1617:
 leaq    32528(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::intermediate_type::IntermediateType>
 jmp     .LBB7390_1501
.LBB7390_1618:
 testb   $1, 117916(%rsp)
 je      .LBB7390_1441
 leaq    29520(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::ast::jodin_node::JodinNode>
 jmp     .LBB7390_1441
.LBB7390_1620:
 leaq    3416(%rsp), %rcx
 leaq    3680(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1621
.LBB7390_1621:
 movq    3416(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1623
 jmp     .LBB7390_1799
.LBB7390_1799:
 jmp     .LBB7390_1624
 ud2
.LBB7390_1623:
 movq    3424(%rsp), %rax
 movq    %rax, 4456(%rsp)
 movq    3432(%rsp), %rax
 movq    %rax, 4464(%rsp)
 movq    3440(%rsp), %rax
 movq    %rax, 4472(%rsp)
 movq    3448(%rsp), %rax
 movq    %rax, 4480(%rsp)
 movq    4456(%rsp), %rax
 movq    %rax, 3376(%rsp)
 movq    4464(%rsp), %rax
 movq    %rax, 3384(%rsp)
 movq    4472(%rsp), %rax
 movq    %rax, 3392(%rsp)
 movq    4480(%rsp), %rax
 movq    %rax, 3400(%rsp)
 jmp     .LBB7390_89
.LBB7390_1624:
 leaq    3424(%rsp), %rdx
 leaq    3944(%rsp), %rcx
 movq    %rcx, 96(%rsp)
 movl    $256, %r8d
 movq    %r8, 104(%rsp)
 callq   memcpy
 movq    96(%rsp), %rdx
 movq    104(%rsp), %r8
 leaq    4200(%rsp), %rcx
 movq    %rcx, 112(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    112(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1625
.LBB7390_1625:
 jmp     .LBB7390_104
.LBB7390_1626:
 jmp     .LBB7390_85
.LBB7390_1627:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1626
.LBB7390_1628:
 leaq    .L__unnamed_428(%rip), %r8
 leaq    113232(%rsp), %rcx
 leaq    113264(%rsp), %rdx
 callq   core::option::Option<T>::unwrap
 jmp     .LBB7390_1629
.LBB7390_1629:
 movb    $1, 117924(%rsp)
 movups  113232(%rsp), %xmm0
 movups  113248(%rsp), %xmm1
 movaps  %xmm1, 113840(%rsp)
 movaps  %xmm0, 113824(%rsp)
 leaq    113864(%rsp), %rcx
 callq   alloc::vec::Vec<T>::new
 jmp     .LBB7390_1630
.LBB7390_1630:
 movq    2792(%rsp), %rdx
 movb    $0, 117924(%rsp)
 leaq    113560(%rsp), %rcx
 leaq    113824(%rsp), %r8
 leaq    113864(%rsp), %r9
 callq   jodin_rs::ast::JodinNodeGenerator::generate_node
 jmp     .LBB7390_1633
.LBB7390_1631:
 testb   $1, 117924(%rsp)
 jne     .LBB7390_1634
 jmp     .LBB7390_85
.LBB7390_1632:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1631
.LBB7390_1633:
 movb    $0, 117924(%rsp)
 leaq    113296(%rsp), %rcx
 leaq    113560(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1635
.LBB7390_1634:
 leaq    113824(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_85
.LBB7390_1635:
 movq    113296(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1637
 jmp     .LBB7390_1800
.LBB7390_1800:
 jmp     .LBB7390_1638
 ud2
.LBB7390_1637:
 movq    113304(%rsp), %rax
 movq    %rax, 114400(%rsp)
 movq    113312(%rsp), %rax
 movq    %rax, 114408(%rsp)
 movq    113320(%rsp), %rax
 movq    %rax, 114416(%rsp)
 movq    113328(%rsp), %rax
 movq    %rax, 114424(%rsp)
 movq    114400(%rsp), %rax
 movq    %rax, 3376(%rsp)
 movq    114408(%rsp), %rax
 movq    %rax, 3384(%rsp)
 movq    114416(%rsp), %rax
 movq    %rax, 3392(%rsp)
 movq    114424(%rsp), %rax
 movq    %rax, 3400(%rsp)
 jmp     .LBB7390_89
.LBB7390_1638:
 leaq    113304(%rsp), %rdx
 leaq    113888(%rsp), %rcx
 movq    %rcx, 72(%rsp)
 movl    $256, %r8d
 movq    %r8, 80(%rsp)
 callq   memcpy
 movq    72(%rsp), %rdx
 movq    80(%rsp), %r8
 leaq    114144(%rsp), %rcx
 movq    %rcx, 88(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    88(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1639
.LBB7390_1639:
 jmp     .LBB7390_104
.LBB7390_1640:
 jmp     .LBB7390_85
.LBB7390_1641:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1640
.LBB7390_1642:
 movq    2288(%rsp), %r8
 movq    2296(%rsp), %rdx
 movq    %rdx, 118200(%rsp)
 movq    %r8, 118208(%rsp)
 leaq    7736(%rsp), %rcx
 callq   core::str::<impl str>::parse
 jmp     .LBB7390_1643
.LBB7390_1643:
 leaq    7472(%rsp), %rcx
 leaq    7736(%rsp), %rdx
 callq   <core::result::Result<T,E> as core::ops::try_trait::Try>::branch
 jmp     .LBB7390_1644
.LBB7390_1644:
 movq    7472(%rsp), %rax
 testq   %rax, %rax
 je      .LBB7390_1646
 jmp     .LBB7390_1801
.LBB7390_1801:
 jmp     .LBB7390_1647
 ud2
.LBB7390_1646:
 movups  7480(%rsp), %xmm0
 movups  7496(%rsp), %xmm1
 movaps  %xmm1, 8528(%rsp)
 movaps  %xmm0, 8512(%rsp)
 movb    $1, 118007(%rsp)
 movaps  8512(%rsp), %xmm0
 movaps  8528(%rsp), %xmm1
 movaps  %xmm1, 7456(%rsp)
 movaps  %xmm0, 7440(%rsp)
 movb    $0, 118007(%rsp)
 movaps  7440(%rsp), %xmm0
 movaps  7456(%rsp), %xmm1
 movaps  %xmm1, 8784(%rsp)
 movaps  %xmm0, 8768(%rsp)
 movaps  8768(%rsp), %xmm0
 movaps  8784(%rsp), %xmm1
 movups  %xmm1, 8576(%rsp)
 movups  %xmm0, 8560(%rsp)
 movb    $1, 8552(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    8552(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_1665
.LBB7390_1647:
 leaq    7480(%rsp), %rdx
 leaq    8000(%rsp), %rcx
 movq    %rcx, 48(%rsp)
 movl    $256, %r8d
 movq    %r8, 56(%rsp)
 callq   memcpy
 movq    48(%rsp), %rdx
 movq    56(%rsp), %r8
 leaq    8256(%rsp), %rcx
 movq    %rcx, 64(%rsp)
 callq   memcpy
 movq    2808(%rsp), %rcx
 movq    64(%rsp), %rdx
 callq   <core::result::Result<T,F> as core::ops::try_trait::FromResidual<core::result::Result<core::convert::Infallible,E>>>::from_residual
 jmp     .LBB7390_1648
.LBB7390_1648:
 movb    $0, 118007(%rsp)
 jmp     .LBB7390_104
.LBB7390_1649:
 jmp     .LBB7390_85
.LBB7390_1650:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1649
.LBB7390_1651:
 testb   $1, 117911(%rsp)
 jne     .LBB7390_1657
 jmp     .LBB7390_1656
.LBB7390_1652:
 movq    3296(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1651
.LBB7390_1653:
 testb   $1, 117911(%rsp)
 jne     .LBB7390_1655
 jmp     .LBB7390_5
.LBB7390_1654:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1653
.LBB7390_1655:
 movb    $0, 117911(%rsp)
 movq    3296(%rsp), %rcx
 callq   alloc::alloc::box_free
 jmp     .LBB7390_5
.LBB7390_1656:
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 testb   $1, 117913(%rsp)
 jne     .LBB7390_1659
 jmp     .LBB7390_1658
.LBB7390_1657:
 movb    $0, 117911(%rsp)
 movq    3296(%rsp), %rcx
 callq   alloc::alloc::box_free
 jmp     .LBB7390_1656
.LBB7390_1658:
 movq    2784(%rsp), %rcx
 movb    $0, 117913(%rsp)
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_1660
.LBB7390_1659:
 leaq    2840(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<[jodin_rs::parsing::Rule]>>
 jmp     .LBB7390_1658
.LBB7390_1660:
 testb   $1, 117914(%rsp)
 jne     .LBB7390_1664
 jmp     .LBB7390_1663
.LBB7390_1661:
 testb   $1, 117914(%rsp)
 jne     .LBB7390_1696
 jmp     .LBB7390_1695
.LBB7390_1662:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1661
.LBB7390_1663:
 movq    2816(%rsp), %rax
 addq    $118232, %rsp
 popq    %rbx
 popq    %rdi
 popq    %rsi
 popq    %r14
 retq
.LBB7390_1664:
 movq    2800(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1663
.LBB7390_1665:
 movb    $0, 118007(%rsp)
 jmp     .LBB7390_89
.LBB7390_1666:
 testb   $1, 118007(%rsp)
 jne     .LBB7390_1668
 jmp     .LBB7390_85
.LBB7390_1667:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1666
.LBB7390_1668:
 leaq    7440(%rsp), %rcx
 callq   core::ptr::drop_in_place<jodin_rs::core::literal::Literal>
 jmp     .LBB7390_85
.LBB7390_1669:
 movq    2304(%rsp), %r8
 movq    2312(%rsp), %rdx
 movq    %rdx, 118216(%rsp)
 movq    %r8, 118224(%rsp)
 leaq    6632(%rsp), %rcx
 callq   <jodin_rs::core::identifier::Identifier as core::convert::From<S>>::from
 jmp     .LBB7390_1670
.LBB7390_1670:
 movups  6632(%rsp), %xmm0
 movups  6648(%rsp), %xmm1
 movaps  %xmm1, 6896(%rsp)
 movaps  %xmm0, 6880(%rsp)
 movaps  6880(%rsp), %xmm0
 movaps  6896(%rsp), %xmm1
 movups  %xmm1, 6688(%rsp)
 movups  %xmm0, 6672(%rsp)
 movb    $2, 6664(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    6664(%rsp), %rdx
 callq   jodin_rs::ast::jodin_node::JodinNode::new
 jmp     .LBB7390_1671
.LBB7390_1671:
 jmp     .LBB7390_89
.LBB7390_1672:
 leaq    7024(%rsp), %rcx
 leaq    7064(%rsp), %rdx
 callq   <I as core::iter::traits::collect::IntoIterator>::into_iter
 jmp     .LBB7390_1673
.LBB7390_1673:
 leaq    6984(%rsp), %rcx
 leaq    7024(%rsp), %rdx
 callq   core::iter::traits::iterator::Iterator::filter
 jmp     .LBB7390_1674
.LBB7390_1674:
 leaq    6944(%rsp), %rcx
 leaq    6984(%rsp), %rdx
 callq   core::iter::traits::iterator::Iterator::map
 jmp     .LBB7390_1675
.LBB7390_1675:
 leaq    6920(%rsp), %rcx
 leaq    6944(%rsp), %rdx
 callq   core::iter::traits::iterator::Iterator::collect
 jmp     .LBB7390_1676
.LBB7390_1676:
 movq    6936(%rsp), %rcx
 movq    %rcx, 7184(%rsp)
 movups  6920(%rsp), %xmm0
 movaps  %xmm0, 7168(%rsp)
 leaq    7136(%rsp), %rcx
 leaq    7168(%rsp), %rdx
 callq   <jodin_rs::core::identifier::Identifier as core::iter::traits::collect::FromIterator<S>>::from_iter
 jmp     .LBB7390_1677
.LBB7390_1677:
 movups  7136(%rsp), %xmm0
 movups  7152(%rsp), %xmm1
 movaps  %xmm1, 7424(%rsp)
 movaps  %xmm0, 7408(%rsp)
 movaps  7408(%rsp), %xmm0
 movaps  7424(%rsp), %xmm1
 movups  %xmm1, 7216(%rsp)
 movups  %xmm0, 7200(%rsp)
 movb    $2, 7192(%rsp)
 leaq    3376(%rsp), %rcx
 leaq    7192(%rsp), %rdx
 callq   <T as core::convert::Into<U>>::into
 jmp     .LBB7390_1678
.LBB7390_1678:
 jmp     .LBB7390_89
.LBB7390_1679:
 jmp     .LBB7390_89
.LBB7390_1680:
 jmp     .LBB7390_89
.LBB7390_1681:
 testb   $1, 117911(%rsp)
 jne     .LBB7390_1683
 jmp     .LBB7390_5
.LBB7390_1682:
 movq    3296(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1681
.LBB7390_1683:
 movb    $0, 117911(%rsp)
 movq    3296(%rsp), %rcx
 callq   alloc::alloc::box_free
 jmp     .LBB7390_5
.LBB7390_1684:
 testb   $1, 117911(%rsp)
 jne     .LBB7390_1691
 jmp     .LBB7390_1690
.LBB7390_1685:
 movq    3296(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pairs::Pairs<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1684
.LBB7390_1686:
 testb   $1, 117911(%rsp)
 jne     .LBB7390_1688
 jmp     .LBB7390_5
.LBB7390_1687:
 movq    %rax, %rcx
 movl    %edx, %eax
 movq    %rcx, 118032(%rsp)
 movl    %eax, 118040(%rsp)
 jmp     .LBB7390_1686
.LBB7390_1688:
 movb    $0, 117911(%rsp)
 movq    3296(%rsp), %rcx
 callq   alloc::alloc::box_free
 jmp     .LBB7390_5
.LBB7390_1689:
 leaq    2840(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<[jodin_rs::parsing::Rule]>>
 jmp     .LBB7390_2
.LBB7390_1690:
 movb    $0, 117911(%rsp)
 movb    $0, 117912(%rsp)
 testb   $1, 117913(%rsp)
 jne     .LBB7390_1693
 jmp     .LBB7390_1692
.LBB7390_1691:
 movb    $0, 117911(%rsp)
 movq    3296(%rsp), %rcx
 callq   alloc::alloc::box_free
 jmp     .LBB7390_1690
.LBB7390_1692:
 movq    2784(%rsp), %rcx
 movb    $0, 117913(%rsp)
 callq   core::ptr::drop_in_place<alloc::vec::Vec<jodin_rs::ast::jodin_node::JodinNode>>
 jmp     .LBB7390_1694
.LBB7390_1693:
 leaq    2840(%rsp), %rcx
 callq   core::ptr::drop_in_place<alloc::boxed::Box<[jodin_rs::parsing::Rule]>>
 jmp     .LBB7390_1692
.LBB7390_1694:
 testb   $1, 117914(%rsp)
 jne     .LBB7390_1697
 jmp     .LBB7390_1663
.LBB7390_1695:
 movq    118032(%rsp), %rcx
 callq   _Unwind_Resume
 ud2
.LBB7390_1696:
 movq    2800(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1695
.LBB7390_1697:
 movq    2800(%rsp), %rcx
 callq   core::ptr::drop_in_place<pest::iterators::pair::Pair<jodin_rs::parsing::Rule>>
 jmp     .LBB7390_1663
.LJTI7390_0:
.LJTI7390_1:
