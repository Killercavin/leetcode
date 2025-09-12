package dvd

class DVD(
    val name: String,
    val releaseYear: Int,
    val director: String
) {
    fun dvdInfo(): String = "$name, directed by $director, released in $releaseYear"
}

/*
The most primitive array operations are writing and reading values into and from it; all the others are built on top of these two operations

Writing data into an array:
- We can choose to write data at any given location in the array using the indexes of the array ie myArray[7] = "Dog"
 */

fun main() {
    val dvdCollection = arrayOfNulls<DVD>(15) // array to hold the DVDs

    // writing data into an array
    val avengersDVD = DVD("The Avengers", 2012, "Jose Whedon" ) // instance of the dvd

    // Next, we'll put it into the 8th place of the Array. Remember, because we started numbering from 0, the index we want is 7.
    dvdCollection[7] = avengersDVD

    // adding more DVDs to our array box of DVDs
    val incrediblesDVD = DVD("The Incredibles", 2004, "Brad Bird")
    val findingDoryDVD = DVD("Finding Dory", 2016, "Andrew Stanton")
    val lionKingDVD = DVD("The Lion King", 2019, "Jon Favreau")

    // Put "The Incredibles" into the 4th place: index 3.
    dvdCollection[3] = incrediblesDVD

    // Put "Finding Dory" into the 10th place: index 9.
    dvdCollection[9] = findingDoryDVD

    // Put "The Lion King" into the 3rd place: index 2.
    dvdCollection[2] = lionKingDVD


    // overwriting items of an array
    // what happens here is the incrediblesDVD is overwritten or modified with the details of starWarsDVD: In real world scenario could be when you remove the avengersDVD and on its space you put the starWarsDVD
    val starWarsDVD = DVD("Star Wars", 1977, "George Lucas")
    dvdCollection[3] = starWarsDVD


    // reading items from an array - real world is like fetching your DVDs from the store box
    // using the loop to access and print all the DVDs in my dvdCollection; notice that for empty indexes `null` is printed
    dvdCollection.forEach { println(it?.dvdInfo()) }
}