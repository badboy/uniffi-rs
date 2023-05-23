import Foundation
import traits

let m = TraitMethods(name: "yo")
assert(String(describing: m) == "TraitMethods(yo)")
assert(String(reflecting: m) == "TraitMethods { val: \"yo\" }")
assert(m == TraitMethods(name: "yo"))
assert(m != TraitMethods(name: "yoyo"))

var d = [TraitMethods:String]()
d[m] = "m"
assert(d.keys.contains(m))
