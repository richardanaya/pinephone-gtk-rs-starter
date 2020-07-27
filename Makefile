run:
	cargo run
build:
	#make the resources
	cd src && glib-compile-resources app.xml
	cargo run
