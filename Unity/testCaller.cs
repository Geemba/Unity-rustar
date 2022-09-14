using rustAstar;
using Toolbox;
using System.Collections;
using Debug = UnityEngine.Debug;
using System.Collections.Generic;
using System.Diagnostics;
using UnityEngine;
using UnityEngine.Tilemaps;

public class testCaller : MonoBehaviour
{
    [SerializeField] Tilemap map;
    [SerializeField] TileBase tile;

    // Update is called once per frame
    void Update()
    {
        if (Input.GetMouseButtonDown(0))
        {
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
        if (Input.GetMouseButtonDown(1))
        {
            foreach (var pos in RustAstar.TestList)
            {
                map.SetTile((Vector3Int)pos,tile);
                print(map.HasTile((Vector3Int)pos));
            }
            var path = AStar.FindPath(map,map.GetCellCenterWorld(new Vector3Int(0, 0, 0)), map.GetCellCenterWorld(new Vector3Int(1, 1, 0)));

            foreach (var item in path)
            {
                print(item);
            }
        }
    }
}
