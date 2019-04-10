import fast_luhn as fl


def test_validate():
    assert fl.validate('5152480083848100') == True
    assert fl.validate('491674804530447') == False

def test_complete():
    assert fl.complete('524402422676340') == '5244024226763402'
    assert fl.complete('530449279124605') == '5304492791246052'

def test_digit():
    assert fl.digit('12345') == 5
    assert fl.digit('') == 0

def test_generate():
    new_string = fl.generate(12)
    assert len(fl.generate(12)) == 12
    assert fl.validate(new_string) == True
