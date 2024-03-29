.version 61 0
.class public super Main
.super java/lang/Object

.method public <init> : ()V
    .code stack 1 locals 1
L0:     aload_0
L1:     invokespecial Method java/lang/Object <init> ()V
L4:     return
L5:
        .linenumbertable
            L0 2
        .end linenumbertable
        .localvariabletable
            0 is this LMain; from L0 to L5
        .end localvariabletable
    .end code
.end method

.method public static main : ([Ljava/lang/String;)V
    .code stack 7 locals 4
        new RIMPInt
dup
ldc "a"
invokespecial Method RIMPInt <init> (Ljava/lang/String;)V
astore 1
new RIMPInt
dup
ldc "b"
invokespecial Method RIMPInt <init> (Ljava/lang/String;)V
astore 2
new RIMPInt
dup
ldc "c"
invokespecial Method RIMPInt <init> (Ljava/lang/String;)V
astore 3
aload 1
ldc 5
invokevirtual Method RIMPInt assign (I)V
aload 2
ldc 10
invokevirtual Method RIMPInt assign (I)V
aload 3
aload 1
invokevirtual Method RIMPInt get ()I
aload 2
invokevirtual Method RIMPInt get ()I
iadd
invokevirtual Method RIMPInt assign (I)V
aload 3
invokevirtual Method RIMPInt print ()V
aload 1
invokevirtual Method RIMPInt print ()V
aload 2
invokevirtual Method RIMPInt print ()V
aload 3
invokevirtual Method RIMPInt unAssign ()V
aload 2
invokevirtual Method RIMPInt unAssign ()V
aload 1
invokevirtual Method RIMPInt unAssign ()V

        return
    .end code
.end method
.sourcefile "Main.java"
.end class
