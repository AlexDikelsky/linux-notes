#asm - assembler and interprater for a minimal computer
#   usage: awk -f asm program-file data-files...

BEGIN {
    srcfile = ARGV[1]
    ARGV[1] = ""  #remaining files are data
    tempfile = "asm.temp"
    n = split("const get put ld st add sub jpos jz j halt", x)
    for (i=1; i<=n; i++)  #This creates table of op codes
	op[x[i]] = i-1

#Pass 1
    FS = "[ \t]+"
    while (getline < srcfile > 0) {
	sub(/#.*/, "") #Remove comments
	symtab[$1] = nextmem
	if ($2 != "") {
	    print $2 "\t" $3 > tempfile
	    nextmem++
	}
    }
    close(tempfile)

#Pass 2
    nextmem = 0
    while (getline <tempfile > 0) {
	if ($2 !~ /^[0-9]*$/) #If symboloc address,
	    $2 = symtab[$2]   #teplace by numeric value
	mem[nextmem++] = 1000 * op[$1] + $2 #pack into word
    }

#Interprate
    for (pc=0; pc>=0; ) {
	addr = mem[pc] % 1000
	code = int(mem[pc++])
	if      (code == op["get"])  { geline acc }
	else if (code == op["put"])  { print acc }
	else if (code == op["st"])   { mem[addr] = acc }
	else if (code == op["ld"])   { acc  = mem[addr] }
	else if (code == op["add"])  { acc += mem[addr] }
	else if (code == op["sub"])  { acc -= mem[addr] }
	else if (code == op["jpos"]) { if (acc >  0) pc = addr }
	else if (code == op["jz"])   { if (acc == 0) pc = addr }
	else if (code == op["j"])    { pc = addr }
	else if (code == op["halt"]) { pc = -1 }
	else                         { pc = -1 }
	}
}
