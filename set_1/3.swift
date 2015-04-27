import Foundation

extension Character {
  func unicodeScalar() -> UnicodeScalar {
    let characterString = String(self)
    let scalars = characterString.unicodeScalars

    return scalars[scalars.startIndex]
  }
}

extension String {
  subscript (i: Int) -> Character {
    return self[advance(self.startIndex, i)]
  }

  subscript (i: Int) -> String {
    return String(self[i] as Character)
  }
}

func hexToInt(hex: String) -> UInt8 {
  return UInt8(strtoul(hex, nil, 16))
}

func englishProbability(string: String) -> Int {
  let probability = "etaoinshrdlu"

  var currentProbability = [Character: Float]()
  var totalScore = 0

  for character in string.lowercaseString {
    if character != Character(" ") && currentProbability[character] == nil {
      var currentCount = 0

      for targetCharacter in string.lowercaseString {
        if targetCharacter == character {
          currentCount++
        }
      }

      currentProbability[character] = Float(currentCount) / Float(count(string))
    }
  }

  let sortedProbability = sorted(currentProbability) { $0.1 > $1.1 }

  for (key, _) in sortedProbability {
    let range = probability.rangeOfString(String(key))

    if let randeIndex = range?.startIndex {
      totalScore += count(probability) - distance(probability.startIndex, randeIndex)
    } else {
      break
    }
  }

  return totalScore
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

func findKey(hex: String) -> Int {
  var scores = [Int: Int]()

  for i in 0..<128 {
    let xorResult = xor(hex, UInt8(i))
    scores[i] = englishProbability(xorResult)
  }

  return sorted(scores) { $0.1 > $1.1 }[0].0
}

func xor(a: String, b: UInt8) -> String {
  var aBytes:[UInt8] = []

  for i in 0..<count(a) / 2 {
    aBytes.append(hexToInt(a[i * 2] + a[i * 2 + 1]))
  }

  var result: String = ""

  for i in 0..<count(aBytes) {
    result += String(UnicodeScalar(aBytes[i] ^ b))
  }

  return result
}

func charToInt(character: Character) -> Int {
  var number = character.unicodeScalar().value - 48
  if number > 9 { number -= 39 }

  return Int(number)
}

let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
let output = findKey(input)

println("result = \(output)")
