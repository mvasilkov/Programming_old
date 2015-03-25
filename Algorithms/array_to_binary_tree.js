/* Given a sorted (increasing order) array with unique integer elements, write
 * an algorithm to create a binary search tree with minimal height. */

var array = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
    61, 67, 71, 73, 79, 83, 89]

function Joint(x) {
    this.value = x
    this.left = null
    this.rigth = null
}

function build(a, first, last) {
    if (last < first)
        return null
    var midpoint = (first + last) >> 1
    var retval = new Joint(a[midpoint])
    retval.left = build(a, first, midpoint - 1)
    retval.right = build(a, midpoint + 1, last)
    return retval
}

var tree = build(array, 0, array.length - 1)

pprint(tree)

function pprint(x) {
    var archy = require('archy')
    console.log(archy(pprintObjectify(x)))
}

function pprintObjectify(x) {
    if (x === null)
        return 'null'
    return {
        label: '' + x.value,
        nodes: [pprintObjectify(x.left), pprintObjectify(x.right)]
    }
}
