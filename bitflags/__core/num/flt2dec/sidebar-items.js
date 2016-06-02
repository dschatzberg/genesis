initSidebarItems({"constant":[["MAX_SIG_DIGITS","The minimum size of buffer necessary for the shortest mode."]],"enum":[["FullDecoded","Decoded unsigned value."],["Part","Formatted parts."],["Sign","Sign formatting options."]],"fn":[["decode","Returns a sign (true when negative) and `FullDecoded` value from given floating point number."],["round_up","When `d[..n]` contains decimal digits, increase the last digit and propagate carry. Returns a next digit when it causes the length change."],["to_exact_exp_str","Formats given floating point number into the exponential form with exactly given number of significant digits. The result is stored to the supplied parts array while utilizing given byte buffer as a scratch. `upper` is used to determine the case of the exponent prefix (`e` or `E`). The first part to be rendered is always a `Part::Sign` (which can be an empty string if no sign is rendered)."],["to_exact_fixed_str","Formats given floating point number into the decimal form with exactly given number of fractional digits. The result is stored to the supplied parts array while utilizing given byte buffer as a scratch. `upper` is currently unused but left for the future decision to change the case of non-finite values, i.e. `inf` and `nan`. The first part to be rendered is always a `Part::Sign` (which can be an empty string if no sign is rendered)."],["to_shortest_exp_str","Formats given floating point number into the decimal form or the exponential form, depending on the resulting exponent. The result is stored to the supplied parts array while utilizing given byte buffer as a scratch. `upper` is used to determine the case of non-finite values (`inf` and `nan`) or the case of the exponent prefix (`e` or `E`). The first part to be rendered is always a `Part::Sign` (which can be an empty string if no sign is rendered)."],["to_shortest_str","Formats given floating point number into the decimal form with at least given number of fractional digits. The result is stored to the supplied parts array while utilizing given byte buffer as a scratch. `upper` is currently unused but left for the future decision to change the case of non-finite values, i.e. `inf` and `nan`. The first part to be rendered is always a `Part::Sign` (which can be an empty string if no sign is rendered)."]],"mod":[["decoder","Decodes a floating-point value into individual parts and error ranges."],["estimator","The exponent estimator."],["strategy","Digit-generation algorithms."]],"struct":[["Decoded","Decoded unsigned finite value, such that:"],["Formatted","Formatted result containing one or more parts. This can be written to the byte buffer or converted to the allocated string."]],"trait":[["DecodableFloat","A floating point type which can be `decode`d."]]});