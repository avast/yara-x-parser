// Example of YARA rule for testing purposes

import "macho"
include "test"

//Rule comment
rule test : bla test
{
	//Rule block comment
	meta:
		author = "Author"
		description = 20
	//String comment
	strings:
		$b = "bar" ascii
	condition:
		$b and not true or true
}
