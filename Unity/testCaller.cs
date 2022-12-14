using rustAstar;
using Debug = UnityEngine.Debug;
using System.Diagnostics;
using UnityEngine;

public class testCaller : MonoBehaviour
{
    // Update is called once per frame
    void Update()
    {
        if (Input.GetMouseButtonDown(0))
        {
            // AllocGraph is called before starting the algorithm 
            // to give the list of walkable tiles to rust
            // it will keep living until you dont call DeallocGraph()
            RustAstar.AllocGraph(RustAstar.TestList, RustAstar.Bounds.Test);
            var st = new Stopwatch();

            st.Start();
            var path = RustAstar.FindPath(new Vector2Int(0, 0), new Vector2Int(10, 10));

            st.Stop();
            Debug.Log(st.Elapsed.TotalMilliseconds);

            //foreach (var item in path)
            //{
            //    Debug.Log(item);
            //}

            RustAstar.DeallocGraph();

            print("Alive");
        }
    }
}
