import java.util.Scanner;

public class HelloWorld {
  private static native String hello(String input);

  static { System.loadLibrary("mylib"); }

  // Run via: `java -Djava.library.path=../../target/release -cp . HelloWorld`
  public static void main(String[] args) {
    try (Scanner in = new Scanner(System.in)) {
      String name = in.nextLine().trim();
      String rustName = hello(name);
      System.out.println("Rust says: " + rustName);
    } catch (Throwable err) {
      System.err.println(err.getStackTrace());
    }
  }
}
