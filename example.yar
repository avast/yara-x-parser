//Global comment

//Rule comment
rule foo
{
	strings:
		$a = "foo"
		$b = "bar"
	condition:
		$a and
		$b
}
