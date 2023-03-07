use std::collections::HashMap;


/*Есть список целых чисел. Создайте функцию, используйте вектор и верните из списка: среднее значение; 
медиану (значение элемента из середины списка после его сортировки); 
моду списка (mode of list, то значение которое встречается в списке наибольшее количество раз; HashMap будет полезна в данном случае).*/



fn main() {
    let mut vector = vec![55,14,88,1,36];

    vector.push(1);
    vector.push(99);
    vector.push(1);
    vector.push(12);


    let res = somefunc(&vector);

    print!("{:?}", res);
}


fn somefunc(vec:  &Vec<i32>) -> HashMap<String,i32>{
    let mut some = HashMap::new();
    let mut summry: i32 = 0;
    for i in vec{
         summry += i;
    }
    summry /= vec.len() as i32;

    some.insert("среднее значение".to_string(), summry);


    some.insert("Медианна".to_string(), vec[vec.len()/2]);

    for i in vec{
        *some.entry(i.to_string()).or_insert(0) +=1;
    }

    some
    
} 


