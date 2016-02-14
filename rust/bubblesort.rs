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
  let n = lista.len()-1;
  let mut temp;
  for x in n..0
   {
    for i in 0..x
     {
      if lista[i]>lista[i+1]
       {
        temp = lista[i];
        lista[i] = lista[i+1];
        lista[i+1] = temp;
       }
     }
   }
 }
