declare void @exit(i32) nounwind

define void @_start() {
    call i64 @main()
    call void @exit(i32 0)
    ret void
}

