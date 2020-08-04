import commands


def test_run_commands():
    """Test for the run_commands function
    """
    commands.run_commands(["ls", "ls"], "Failed to run test")
    # We need an assert so as long as the code above doesn't exit then the
    # test should pass
    assert True == True
