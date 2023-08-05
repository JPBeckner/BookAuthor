dev:
	@echo "Starting server local for development\n"
	@cargo watch -x 'run'

up:
	@sudo service mongod start
	@sudo service mongod status
