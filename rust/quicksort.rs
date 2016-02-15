fn main()
 {
  let mut lista: [i32;9] = [54,26,93,17,77,31,44,55,20];
  quicksort(&mut lista);
  for i in 0..9
   {
    println!("{}",lista[i]);
   }
 }
fn quicksort(mut lista: &mut [i32])
 {
  let l = lista.len()-1;
  let t = l as i32;
  quicksorth(&mut lista,0,t);
 }
fn quicksorth(mut lista: &mut [i32], f: usize, l: i32)
 {
  let f1 = f as i32;
  let l1 = l as usize;
  if f1 < l
   {
    let s=particion(&mut lista, f, l1);
    let s1 = s as i32;
    let n1 = s1 - 1;
    let x: usize = s+1;
    quicksorth(&mut lista, f, n1);
    quicksorth(&mut lista, x, l);
   }
 }
fn particion(mut lista: &mut [i32], f: usize, l: usize) -> usize
 {
  let pivot = lista[f];
  
  let mut izq = f+1;
  let mut der = l;
  let mut d = false;
  let mut temp;
  
  while !d
   {
    while izq <= der && lista[izq] <= pivot
     {
      izq = izq+1;
     }
    while lista[der] >= pivot && der >= izq
     {
      der=der-1;
     }
    if der < izq
     {
      d=true;
     }
    else
     {
      temp = lista[izq];
      lista[izq] = lista[der];
      lista[der] = temp;
     }
   }
  temp = lista[f];
  lista[f] = lista[der];
  lista[der] = temp;
  
  return der;
 }
