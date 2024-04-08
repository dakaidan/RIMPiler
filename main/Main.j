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
    .code stack 20 locals 7
        new RIMPInt
dup
ldc "n"
invokespecial Method RIMPInt <init> (Ljava/lang/String;)V
astore 1
new RIMPInt
dup
ldc "collatz"
invokespecial Method RIMPInt <init> (Ljava/lang/String;)V
astore 2
new RIMPInt
dup
ldc "generated_name_semantic_transformer0"
invokespecial Method RIMPInt <init> (Ljava/lang/String;)V
astore 3
new RIMPInt
dup
ldc "q"
invokespecial Method RIMPInt <init> (Ljava/lang/String;)V
astore 4
new RIMPInt
dup
ldc "p"
invokespecial Method RIMPInt <init> (Ljava/lang/String;)V
astore 5
new RIMPInt
dup
ldc "r"
invokespecial Method RIMPInt <init> (Ljava/lang/String;)V
astore 6
aload 1
ldc 8
invokevirtual Method RIMPInt assign (I)V
aload 2
ldc 0
invokevirtual Method RIMPInt assign (I)V
aload 3
ldc 0
invokevirtual Method RIMPInt assign (I)V
LSTART0:
aload 1
invokevirtual Method RIMPInt get ()I

ldc 1

if_icmple LENDLOOP1
aload 4
aload 1
invokevirtual Method RIMPInt get ()I
ldc 2
idiv
invokevirtual Method RIMPInt assign (I)V
aload 5
aload 4
invokevirtual Method RIMPInt get ()I
ldc 2
imul
invokevirtual Method RIMPInt assign (I)V
aload 6
aload 1
invokevirtual Method RIMPInt get ()I
aload 5
invokevirtual Method RIMPInt get ()I
isub
invokevirtual Method RIMPInt assign (I)V
aload 6
invokevirtual Method RIMPInt get ()I

ldc 0

if_icmpne LELSE2
aload 1
aload 1
invokevirtual Method RIMPInt get ()I
ldc 2
idiv
invokevirtual Method RIMPInt assign (I)V
goto LENDELSE3
LELSE2:
aload 1
ldc 3
aload 1
invokevirtual Method RIMPInt get ()I
imul
ldc 1
iadd
invokevirtual Method RIMPInt assign (I)V

LENDELSE3:
aload 3
aload 3
invokevirtual Method RIMPInt get ()I
ldc 1
iadd
invokevirtual Method RIMPInt assign (I)V
goto LSTART0
LENDLOOP1:
aload 2
ldc 1
invokevirtual Method RIMPInt assign (I)V
aload 3
invokevirtual Method RIMPInt print ()V
aload 6
invokevirtual Method RIMPInt print ()V
aload 2
invokevirtual Method RIMPInt print ()V
aload 4
invokevirtual Method RIMPInt print ()V
aload 1
invokevirtual Method RIMPInt print ()V
aload 5
invokevirtual Method RIMPInt print ()V
aload 2
invokevirtual Method RIMPInt unAssign ()V
LSTART4:
aload 3
invokevirtual Method RIMPInt get ()I

ldc 0

if_icmple LENDLOOP5
aload 3
invokevirtual Method RIMPInt unAssign ()V
aload 6
invokevirtual Method RIMPInt get ()I

ldc 0

if_icmpne LELSE6
aload 1
invokevirtual Method RIMPInt unAssign ()V
goto LENDELSE7
LELSE6:
aload 1
invokevirtual Method RIMPInt unAssign ()V

LENDELSE7:
aload 6
invokevirtual Method RIMPInt unAssign ()V
aload 5
invokevirtual Method RIMPInt unAssign ()V
aload 4
invokevirtual Method RIMPInt unAssign ()V
goto LSTART4
LENDLOOP5:
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
