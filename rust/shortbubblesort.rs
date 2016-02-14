fn main()
{
 let mut lista: [i32;10] = [20,30,40,90,50,60,70,80,100,110];
 bubblesort(&mut lista);
 for i in 0..10
  {
   println!("{}",lista[i]);
  }
}
fn bubblesort(lista: &mut [i32])
 {
  let mut c = true;
  let mut n = lista.len()-1;
  let mut temp;
  while n > 0 && c == true
   {
    c = false;
    for i in 0..n
     {
      if lista[i]>lista[i+1]
       {
        c = true;
        temp = lista[i];
        lista[i] = lista[i+1];
        lista[i+1] = temp;
       }
     }
    n = n-1;
   }
 }
