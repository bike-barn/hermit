URCHIN=`which urchin`
TEST_SUITE=fast

.PHONY: bash test

test:
	@$(URCHIN) -f test/slow

bash:
	@printf '\n\033[0;34m%s\033[0m\n' "Running tests in $@"
	@$@ $(URCHIN) -f test/$(TEST_SUITE)
