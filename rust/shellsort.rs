fn main() 
 {
  let mut lista: [i32;9] = [54,26,93,17,77,31,44,55,20];
  shellsort(&mut lista);
  for i in 0..9
   {
    println!("{}",lista[i]);
   }
 }
fn shellsort(mut lista: &mut [i32])
 {
  let mut c = lista.len()/2;
  while c > 0
   {
    for s in 0..c
     {
      insercion(&mut lista, s, c);
     }
    c = c/2;
   }
 }
fn insercion(mut lista: &mut [i32], i: usize, g: usize)
 {
  let x = i+g;
  let n = lista.len();
  for i in x..n
   {
    let valor=lista[i];
    let mut posicion=i;
    while posicion>=g && lista[posicion-g]>valor
     {
      lista[posicion] = lista[posicion-g];
      posicion = posicion-g;
     }
    lista[posicion]=valor;
   }
 }
