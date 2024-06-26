import nnsplit_unblocked


def test_basic():
    assert 1 + 1 == 2


def test_nnsplit():
    splitter = nnsplit_unblocked.NNSplit()
    output = splitter.split(["This is a test. This is another test."])
    assert output == [["This is a test. ", "This is another test."]]


def test_same_api():
    splitter = nnsplit_unblocked.NNSplit.load("en")
    text = "This is a test. This is another test."
    splits = list(splitter.split([text])[0])
    print(splits)
    assert splits == ["This is a test. ", "This is another test."]
