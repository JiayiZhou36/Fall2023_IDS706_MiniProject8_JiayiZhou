rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

install:
	# Install if needed
	#@echo "Updating rust toolchain"
	#rustup update stable
	#rustup default stable 

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release

all: format lint test run

# Custom tasks

# Example: Extract data
extract: 
	cargo run extract

# Example: Transform and Load data
transform_load:
	cargo run transform_load

# Example: Create a database entry
create:
	cargo run query "INSERT INTO Goose (name, year, team, league, goose_eggs, broken_eggs, mehs, league_average_gpct, ppf, replacement_gpct, gwar, key_retro) VALUES ('Emma Watson', 1931, 'BSN', 'NL', 1, 0, 1, 0.67842, 99.0, 0.7342619, 0.1699602, 'watse101');"

# Example: Read from the database
read:
	cargo run query "SELECT * FROM Goose WHERE name='Emma Watson';"

# Example: Update a database entry
update:
	cargo run query "UPDATE Goose SET name='Emma Watson', year=1931, team='BSN', league='NL', goose_eggs=1, broken_eggs=0, mehs=1, league_average_gpct=0.769262, ppf=99.0, replacement_gpct=0.7342619, gwar=0.1699602, key_retro='watse101' WHERE id=1;"

# Example: Delete a database entry
delete:
	cargo run query "DELETE FROM Goose WHERE id=1;"
	
python_install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

python_test:
	python -m pytest -vv --cov=main --cov=mylib test_*.py

python_format:	
	black *.py 

python_lint:
	ruff check *.py mylib/*.py

python_container-lint:
	docker run --rm -i hadolint/hadolint < Dockerfile

python_refactor: format lint

python_deploy:
	#deploy goes here
		
python_all: install lint test format deploy

generate_and_push:
	# Add, commit, and push the generated files to GitHub
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local user.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add metric log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi
