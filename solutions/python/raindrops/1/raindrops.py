def convert(number):
    substitutions = {3: "Pling", 5: "Plang", 7: "Plong"}
    converted = "".join(
        substitutions[key] for key in substitutions if number % key == 0
    )
    if len(converted) == 0:
        converted = str(number)
    return converted
