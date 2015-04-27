extension Character {
  func unicodeScalar() -> UnicodeScalar {
    let characterString = String(self)
    let scalars = characterString.unicodeScalars

    return scalars[scalars.startIndex]
  }
}

func xor(a: String, b: String) -> String {
  let aBytes:[Int] = map(a, charToInt)
  let bBytes:[Int] = map(b, charToInt)

  var result: String = ""

  for i in 0..<count(aBytes) {
    result += String(aBytes[i] ^ bBytes[i], radix: 16)
  }

  return result
}

func charToInt(character: Character) -> Int {
  var number = character.unicodeScalar().value - 48
  if number > 9 { number -= 39 }

  return Int(number)
}

let input1 = "1c0111001f010100061a024b53535009181c"
let input2 = "686974207468652062756c6c277320657965"
let expectedResult = "746865206b696420646f6e277420706c6179"
let output = xor(input1, input2)

println("result = \(output)")
println(output == expectedResult)
