# Inspiration:
# - https://devhints.io/makefile
# - https://tech.davis-hansson.com/p/make/
# - https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html

# Default - top level rule is what gets run when you run just 'make' without specifying a goal/target.
.DEFAULT_GOAL := doc

# Make will delete the target of a rule if it has changed and its recipe exits with a nonzero exit status, just as it
# does when it receives a signal.
.DELETE_ON_ERROR:

# When a target is built, all lines of the recipe will be given to a single invocation of the shell rather than each
# line being invoked separately.
.ONESHELL:

# If this variable is not set, the program '/bin/sh' is used as the shell.
SHELL := bash

# The default value of .SHELLFLAGS is -c normally, or -ec in POSIX-conforming mode.
# Extra options are set for Bash:
#   -e             Exit immediately if a command exits with a non-zero status.
#   -u             Treat unset variables as an error when substituting.
#   -o pipefail    The return value of a pipeline is the status of the last command to exit with a non-zero status,
#                  or zero if no command exited with a non-zero status.
.SHELLFLAGS := -euo pipefail -c

# Eliminate use of Make's built-in implicit rules.
MAKEFLAGS += --no-builtin-rules

# Issue a warning message whenever Make sees a reference to an undefined variable.
MAKEFLAGS += --warn-undefined-variables

# Bring in variables from the '.env' file, ignoring errors if it does not exist.
-include .env

# Export all variables to child processes by default.
# This is used to bring forward all of the values that have been set in the '.env' file included above.
.EXPORT_ALL_VARIABLES:

# Check that the version of Make running this file supports the .RECIPEPREFIX special variable.
# We set it to '>' to clarify inlined scripts and disambiguate whitespace prefixes.
# All script lines start with "> " which is the angle bracket and one space, with no tabs.
ifeq ($(origin .RECIPEPREFIX), undefined)
  $(error This Make does not support .RECIPEPREFIX. Please use GNU Make 4.0 or later.)
endif

.RECIPEPREFIX = >

# GNU make knows how to execute several recipes at once.
# Normally, make will execute only one recipe at a time, waiting for it to finish before executing the next.
# However, the '-j' or '--jobs' option tells make to execute many recipes simultaneously.
# With no argument, make runs as many recipes simultaneously as possible.
MAKEFLAGS += --jobs

# Configure an 'all' target to cover the bases.
all: test lint build doc ## Test and lint and build and document.
.PHONY: all

binary_name := $(shell basename $(CURDIR))
image_repository := jlucktay/$(binary_name)

# Adjust the width of the first column by changing the '-20s' value in the printf pattern.
help:
> @grep -E '^[a-zA-Z0-9_-]+:.*? ## .*$$' $(filter-out .env, $(MAKEFILE_LIST)) | sort \
  | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
.PHONY: help

# Set up some lazy initialisation functions to find code files, so that targets using the output of '$(shell ...)' only
# execute their respective shell commands when they need to, rather than every single instance of '$(shell ...)' being
# executed every single time 'make' is run for any target and wasting a lot of time.
# Further reading at https://www.oreilly.com/library/view/managing-projects-with/0596006101/ch10.html under the 'Lazy
# Initialization' heading.
find-rust-files = $(shell find $1 -name target -prune -or -type f \( -iname '*.rs' \
  -or -name Cargo.toml -or -name Cargo.lock \))

RUST_FILES = $(redefine-rust-files) $(RUST_FILES)

redefine-rust-files = $(eval RUST_FILES := $(call find-rust-files, .))

# Tests look for sentinel files to determine whether or not they need to be run again.
# If any code file has been changed since the sentinel file was last touched, it will trigger a retest.
test: tmp/.tests-passed.sentinel ## Run tests.

# Linter checks look for sentinel files to determine whether or not they need to check again.
# If any code file has been changed since the sentinel file was last touched, it will trigger a rerun.
lint: tmp/.linted.sentinel ## Lint all of the code. Will also test.

# Builds look for image ID files to determine whether or not they need to build again.
# If any code file has been changed since the image ID file was last touched, it will trigger a rebuild.
build: tmp/.built.sentinel ## [DEFAULT] Build the code. Will also test and lint.

# Generates the documentation for the crate.
doc: tmp/.doc.sentinel ## Generate crate docs.

.PHONY: all test lint build doc

clean: ## Clean up the built assets, test coverage, and the temp sub-directory.
> cargo clean
> rm -rf tmp
.PHONY: clean

# Tests - re-run if any code files have changes since 'tmp/.tests-passed.sentinel' was last touched.
tmp/.tests-passed.sentinel: $(RUST_FILES)
> mkdir -p $(@D)
> cargo test
> cargo test --all-features
> cargo test --no-default-features
> touch $@

# Lint - re-run if the tests have been re-run (and so, by proxy, whenever the source files have changed).
# These checks are all read-only and will not make any changes.
tmp/.linted.sentinel: tmp/.linted.fmt.sentinel tmp/.linted.clippy.sentinel
> mkdir -p $(@D)
> touch $@

tmp/.linted.fmt.sentinel: tmp/.tests-passed.sentinel
> mkdir -p $(@D)
> cargo fmt --all --check \
  | awk '{ print } END { if (NR != 0) { print "Please run \"make fmt\" to fix formatting issues!"; exit 1 } }'
> touch $@

fmt: ## Runs 'rustfmt' to format all code.
> cargo fmt --all
.PHONY: fmt

tmp/.linted.clippy.sentinel: tmp/.tests-passed.sentinel
> mkdir -p $(@D)
> cargo clippy -- --warn clippy::pedantic --warn clippy::nursery --warn clippy::unwrap_used \
  --warn clippy::expect_used --forbid unsafe_code
> touch $@

tmp/.built.sentinel: tmp/.linted.sentinel
> mkdir -p $(@D)
> cargo build --all-features --release
> touch $@

tmp/.doc.sentinel: tmp/.built.sentinel
> mkdir -p $(@D)
> cargo doc --all-features --document-private-items --no-deps
> touch $@
