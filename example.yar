import "macho"
include "test"
//Global comment

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
