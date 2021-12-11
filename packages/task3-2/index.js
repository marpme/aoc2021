import { readFileSync } from 'fs'

const binLines = readFileSync('./input2.txt').toString().split('\n')

const setLength = binLines.length

const getMostCommon = (rate, size) => String(rate / size < 0.5 ? 0 : 1)
const getLeastCommon = (rate, size) => String(rate / size >= 0.5 ? 0 : 1)

const calculateRate = (binaries, rater) => {
    const binaryLength = binaries[0].length

    let rate = new Array(binaryLength).fill(0)
    let result = new Array(binaryLength).fill('0')
    let copyBinaries = Array.from(binaries)

    for (let column = 0; column < binaryLength; column++) {
        for (let bin of copyBinaries) {
            rate[column] += bin[column] === '1'
        }

        const mostCommonBit = rater(rate[column], copyBinaries.length)
        copyBinaries = copyBinaries.filter((bin) => {
            return bin[column] === mostCommonBit
        })
        result[column] = mostCommonBit

        // console.log('mcb:', mostCommonBit)
        // console.log('rate:', rate)
        // console.log('result:', result)
        // console.log('copy:', copyBinaries)

        if (copyBinaries.length == 1) {
            return parseInt(copyBinaries[0], 2)
        }
    }

    return parseInt(result.join(''), 2)
}

const mostCommon = calculateRate(binLines, getMostCommon)
const leastCommon = calculateRate(binLines, getLeastCommon)

console.log('O2 rating', mostCommon * leastCommon)
