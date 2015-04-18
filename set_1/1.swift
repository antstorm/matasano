import Darwin

extension Character {
  func unicodeScalar() -> UnicodeScalar {
    let characterString = String(self)
    let scalars = characterString.unicodeScalars

    return scalars[scalars.startIndex]
  }
}

func charToInt(character: Character) -> Int {
  var number = character.unicodeScalar().value - 48
  if number > 9 { number -= 39 }

  return Int(number)
}

func intToBase64(number: Int) -> Character {
  if number < 26 {
    return Character(UnicodeScalar(number + 65))
  } else if number < 52 {
    return Character(UnicodeScalar(number + 71))
  } else if number < 62 {
    return Character(UnicodeScalar(number - 4))
  } else if number == 62 {
    return "+"
  } else {
    return "/"
  }
}

func hexToBase64(hex: String) -> String {
  var result: String = ""

  for i in 0..<(count(hex) / 3) {
    let leftPart = charToInt(Array(hex)[i * 3])
    let middlePart = charToInt(Array(hex)[i * 3 + 1])
    let rightPart = charToInt(Array(hex)[i * 3 + 2])

    // 4 bits from the left part and first 2 bits from the middle part
    let firstCharacter = leftPart << 2 + middlePart >> 2
    // last 2 bits from the middle part and 4 bits from the right part
    let lastCharacter = (middlePart & 0x3) << 4 + rightPart

    result += String(intToBase64(firstCharacter))
    result += String(intToBase64(lastCharacter))
  }

  return result
}

let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
let expectedResult = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
let output = hexToBase64(input)

println("result = \(output)")
println(output == expectedResult)
