BEGIN {
    FS = "\t+"
}

{
    printf("%s {\n\tprintf(\"line %%d, %s: %%s\\n\", NR, $0) }\n",
	   $1, $2)
}
