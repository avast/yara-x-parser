//Global comment

//Rule comment
rule test
{
	//Rule block comment

	//String comment
	strings:
		$a = "foo"
		$b = "bar"
	condition:
		$b and not true or false
}
