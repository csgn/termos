# termos

Terminal Option Select Menu

```no-run

Lorem ipsum dolor sit amet: [y/N]
	press y or N
	N is default
	if you press enter it will be N
	
	


	
```

```rust
let mut menu = Termos::Menu::init();
menu[0].add_question("Lorem ipsum dolor sit amet", Termos::Variant::Choice);
menu[0].add_option(key=["y", "Y"], default=true);
menu[0].add_option(key=["n", "N"]);

menu[1].add_question("Lorem ipsum dolor sit amet 2", Termos::Variant::Choice);
menu[1].add_option(key=["e", "E"]);
menu[1].add_option(key=["h", "H"], default=true);


// OUTPUT:
Lorem ipsum dolor sit amet: [Y/n]

Lorem ipsum dolor sit amet: [e/H]

```
