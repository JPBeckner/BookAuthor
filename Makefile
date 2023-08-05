dev:
	@echo "Starting server local for development\n"
	@cargo watch -x 'run'

mongo:
	@sudo service mongod start
	@sudo service mongod status
