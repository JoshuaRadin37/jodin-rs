# js-like implementation

```c
class Node<T> {
    private data: T;
    private before: *Node<T>;
    private after: *Node<T>;
	
	public Node(data: T) {
		this.data = data;
	}
}
```


```js

function Node(data, before, after) {
    return {
        "data": data,
        "before": before,
        "after": after,
    }
}

```