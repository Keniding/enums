pub fn options() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("{:?}, {:?}, {:?}", some_number, some_char, absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // No se puede sumar diferentes tipos y menos si es posible nulo
    // let sum = x + y;
    let sum = x + y.unwrap_or(0);
    println!("sum = {}", sum);
}

/*
    La Option<T>enumeración es tan útil que incluso se incluye en el preludio; no es necesario incluirla explícitamente en el ámbito. Sus variantes también se incluyen en el preludio: se puede usar "Some" y None directamente sin el Option:: prefijo. La Option<T>enumeración sigue siendo una enumeración normal, y " Some(T)y" None siguen siendo variantes del tipo Option<T>.
    La <T>sintaxis es una característica de Rust que aún no hemos abordado. Se trata de un parámetro de tipo genérico, y los abordaremos con más detalle en el Capítulo 10. Por ahora, solo necesita saber que esto <T>significa que la Some variante de la Option enumeración puede contener un dato de cualquier tipo, y que cada tipo concreto que se usa en su lugar T convierte el tipo general Option<T>en un tipo diferente. Aquí hay algunos ejemplos del uso de Option valores para contener tipos numéricos y char:a
 */
// Rust ya lo tiene integrado el Options en su preludio, comentarlo para verlo
/*
enum Option<T> {
    None,
    Some(T),
}
 */

/*
El tipo de some_number es Option<i32>. El tipo de some_char es Option<char>, que es un tipo diferente. Rust puede inferir estos tipos porque hemos especificado un valor dentro de la Some variante. Para absent_number, Rust requiere que anotemos el Option tipo general: el compilador no puede inferir el tipo que Some contendrá la variante correspondiente observando solo un None valor. Aquí, le indicamos a Rust que queremos que absent_number for sea del tipo Option<i32>.

Cuando tenemos un Some valor, sabemos que existe y que se guarda en el [nombre del objeto Some]. Tener un None valor, en cierto sentido, significa lo mismo que tener un valor nulo: no tenemos un valor válido. Entonces, ¿por qué es mejor tener Option<T> un valor que tener un valor nulo?

En resumen, dado que Option<T>y T(donde T puede ser cualquier tipo) son tipos diferentes, el compilador no nos permitirá usar un Option<T>valor como si fuera definitivamente válido. Por ejemplo, este código no compila porque intenta agregar un i8a un Option<i8>

¡Intenso! En efecto, este mensaje de error significa que Rust no entiende cómo sumar an i8y an Option<i8>, porque son de tipos diferentes. Cuando tenemos un valor de un tipo, como i8en Rust, el compilador se asegurará de que siempre tengamos un valor válido. Podemos proceder con confianza sin tener que comprobar si es nulo antes de usar ese valor. Solo cuando tenemos an Option<i8>(o cualquier otro tipo de valor con el que estemos trabajando) debemos preocuparnos por la posibilidad de no tener un valor, y el compilador se asegurará de que gestionemos ese caso antes de usarlo.

En otras palabras, debes convertir un valor Option<T>a a Tantes de poder realizar T operaciones con él. Generalmente, esto ayuda a detectar uno de los problemas más comunes con null: asumir que algo no es nulo cuando en realidad lo es.

Eliminar el riesgo de asumir incorrectamente un valor distinto de nulo te ayuda a tener más confianza en tu código. Para tener un valor que pueda ser nulo, debes aceptarlo explícitamente estableciendo el tipo de ese valor como Option<T>. Luego, al usar ese valor, debes gestionar explícitamente el caso en que el valor sea nulo. Siempre que un valor tenga un tipo distinto de Option<T>, puedes asumir con seguridad que no es nulo. Esta fue una decisión de diseño deliberada de Rust para limitar la omnipresencia de los valores nulos y aumentar la seguridad del código de Rust.

Entonces, ¿cómo se obtiene el T valor de una Some variante cuando se tiene un valor de tipo Option<T>para poder usarlo? La Option<T>enumeración tiene una gran cantidad de métodos útiles en diversas situaciones; puedes consultarlos en su documentación. Familiarizarse con los métodos de [enlace faltante] Option<T>te será extremadamente útil en tu experiencia con Rust.

En general, para usar un Option<T>valor, se necesita código que gestione cada variante. Se necesita código que se ejecute solo cuando se tenga un Some(T)valor y este código pueda usar el valor interno T. Se necesita otro código que se ejecute solo si se tiene un None valor y este código no tiene ningún T valor disponible. La match expresión es una
 */