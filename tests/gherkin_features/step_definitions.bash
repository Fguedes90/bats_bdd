#!/usr/bin/env bash
# Wrapper step_definitions.bash for gherkin_features tests
# This file sources all step definition files in this directory

# Get the directory of this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source background steps
source "$SCRIPT_DIR/background_steps.bash"

# Source outline steps
source "$SCRIPT_DIR/outline_steps.bash"

# Source tags steps
source "$SCRIPT_DIR/tags_steps.bash"

# Source i18n steps
source "$SCRIPT_DIR/i18n_steps.bash"

# Source step keywords steps
source "$SCRIPT_DIR/step_keywords_steps.bash"

# Source outline simple steps
source "$SCRIPT_DIR/outline_simple_steps.bash"
