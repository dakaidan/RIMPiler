import java.util.Stack;

public class RIMPInt {
    String name;
    int value;
    Stack<Integer> history;

    boolean debug = false;

    public RIMPInt(String name) {
        value = 0;
        history = new Stack<>();
        history.push(0);
        this.name = name;
        // check if debug environment variable is set
        String debugEnv = System.getenv("RIMP_DEBUG");
        if (debugEnv != null && debugEnv.equals("1")) {
            debug = true;
        }
        if (debug) {
            System.out.println("Creating new RIMPInt: " + this.name);
        }
    }

    public void assign(int value) {
        if (debug) {
            System.out.println("Assigning " + this.name + " to " + value + " new size: " + (this.history.size() + 1));
        }
        this.history.push(value - this.value);
        this.value = value;
    }

    public void unAssign() {
        if (debug) {
            if (this.history.isEmpty()) {
                System.out.println("Unassigning " + this.name + " failed: history is empty");
            } else {
                System.out.println("Unassigning " + this.name + " to " + (this.value - this.history.peek()) + " remaining assignments: " + (this.history.size() - 1));
            }
        }
        this.value = this.value - this.history.peek();
        this.history.pop();
    }

    public int get() {
        if (debug) {
            System.out.println("Getting " + this.name + " value: " + this.value);
        }
        return this.value;
    }

    public void print() {
        System.out.print(this.name + ": " + this.value + "\t [");
        for (Integer integer : this.history) {
            System.out.print(integer + " ");
        }
        System.out.println("]");
    }
}

