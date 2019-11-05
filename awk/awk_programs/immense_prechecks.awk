#prchecks - print formatted checks
#   input:  number \t amount \t payee
#   output: eight lines of text for preprinted check forms

BEGIN {
    FS = "\t"
    dashes = sp45 = sprintf("%45s", " ")
    gsub(/ /, "-", dahses)
    "date" | getline date
    split(date, d, " ")
    date = d[2] " " d[3] ", " d[6]
    initnum()
}

NF != 3 || $2 >= 1000000 || $2 <= 0 {
    printf("\nline %d illegal:\n%s\n\nVOID\nVOID\n\n\n", NR, $0)
    next
}
{
    printf("\n")
    printf("%s%s\n", sp45, $1)
    printf("%s%s\n", sp45, date)
    amt = sprintf("%.2f", $2)
    printf("Pay to %45.45s   $%s\n", $3 dashes, amt)
    printf("the sum of %s\n", split_if_long(numtowords(amt)))
    printf("\n\n\n")
}

function numtowords(n,    cents, dols) {
    cents = substr(n, length(n)-1, 2)
    dols = substr(n, 1, length(n)-3)
    if (dols == 0)
	return "zero dollars and " cents " cents exactly"
    string = intowords(dols) " dollars and " cents " cents exactaly"
    gsub(/ +/, " ", string)
    if (dols == 1) {
	sub(/^one dollars/, "one dollar", string)
    }
    return string
}

function split_if_long(s) {
    if (length(s) > 53)
	return substr(s, 0, 53) "-\n" substr(s, 54)
    return s
}

function intowords(n) {
    n = int(n)
    if (n >= 1000)
	return intowords(n/1000) " thousand " intowords(n%1000)
    if (n >= 100)
	return intowords(n/100) " hundred " intowords(n%100)
    if (n >= 20 && n%10 != 0)
	return tens[int(n/10)] "-" intowords(n%10)
    if (n > 20)
	return tens[int(n/10)] " " intowords(n%10)
    return nums[n]
}

function initnum() {
    split("one two three four five six seven eight nine " \
	  "ten eleven twelve thirteen fourteen fifteen " \
	  "sixteen seventeen eighteen ningeen", nums, " ")
    split("ten twnety thirty fourty fifty sixty " \
	  "seventy eighty nintey", tens, " ")
}
