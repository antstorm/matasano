import Foundation

class ByteArray: Printable {
  var bytes:[UInt8] = []

  var description: String {
    return bytes.description
  }

  init() {}

  init(hex: String) {
    for i in 0..<count(hex) / 2 {
      let firstDigit = String(hex[advance(hex.startIndex, i * 2)])
      let secondDigit = String(hex[advance(hex.startIndex, i * 2 + 1)])

      bytes.append(hexToInt(firstDigit + secondDigit))
    }
  }

  func xor(input: UInt8) -> ByteArray {
    var result = ByteArray()

    for byte in bytes {
      result.append(byte ^ input)
    }

    return result
  }

  func append(byte: UInt8) {
    bytes.append(byte)
  }

  func hexToInt(hex: String) -> UInt8 {
    return UInt8(strtoul(hex, nil, 16))
  }

  // Output

  func toHex() -> String {
    var result = ""

    for byte in bytes {
      result += String(byte, radix: 16)
    }

    return result
  }

  func toString() -> String {
    var result = ""

    for byte in bytes {
      result += String(UnicodeScalar(byte))
    }

    return result
  }
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

func findKey(hex: String) -> Int {
  let bytes = ByteArray(hex: hex)

  var scores = [Int: Int]()

  for i in 0..<256 {
    let xorResult = bytes.xor(UInt8(i)).toString()
    scores[i] = englishProbability(xorResult)
  }

  return sorted(scores) { $0.1 > $1.1 }[0].0
}

let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
let output = findKey(input)

println("result = \(output)")
