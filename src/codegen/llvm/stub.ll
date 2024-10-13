declare void @exit(i32) nounwind

define void @_start() {
    %_exit_code = call i32 @main()
    call void @exit(i32 %_exit_code)
    ret void
}

