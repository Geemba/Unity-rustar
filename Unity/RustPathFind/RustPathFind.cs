using System;
using System.Collections.Generic;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Security;
using System.Threading.Tasks;
using UnityEngine.Tilemaps;
using Vector2Int = UnityEngine.Vector2Int;
using Vector3Int = UnityEngine.Vector3Int;

namespace rustAstar
{
    public static class RustAstar
    {
        internal static IntPtr graphPtr;

        [StructLayout(LayoutKind.Sequential)]
        public struct Bounds
        {
            public int x_min;
            public int y_min;
            public int x_max;
            public int y_max;

            public Bounds(UnityEngine.BoundsInt bounds)
            {
                this.x_min = bounds.xMin;
                this.y_min = bounds.yMin;
                this.x_max = bounds.xMax;
                this.y_max = bounds.yMax;
            }

            public static Bounds TestParam(int xMax, int yMax)
            {
                return new Bounds
                {
                    x_min = 0,
                    y_min = 0,
                    x_max = xMax,
                    y_max = yMax,
                };
            }
            public static Bounds Test
            {
                get => TestParam(11, 11);
            }
        }

        /// <summary>
        /// Creates a grid of size (width, height) for testing
        /// </summary>
        public static List<Vector2Int> TestListParam(int width, int height)
        {
            var list = new List<Vector2Int>();

            for (int x = 0; x <= width; x++)
            {
                for (int y = 0; y <= height; y++)
                {
                    list.Add(new Vector2Int(x, y));
                }
            }
            return list;
        }

        public static List<Vector2Int> TestList
        {
            get => TestListParam(10, 10);
        }

        public static void AllocGraph(Tilemap map, List<Vector2Int> list)
        {
            AllocGraph(list, new Bounds(map.cellBounds));
        }
        public static void AllocGraph(Tilemap map, List<Vector3Int> list)
        {
            AllocGraph(list, new Bounds(map.cellBounds));
        }

        public static void AllocGraph(List<Vector3Int> list, Bounds bounds)
        {
            var wrap = create_vec();

            for (int i = 0; i < list.Count; i++)
            {
                add_to_vec(wrap, ToRustVector2((Vector2Int)list[i]));
            }

            graphPtr = allocate_graph(wrap, bounds);
        }
        public static void AllocGraph(List<Vector2Int> list, Bounds bounds)
        {
            var wrap = create_vec();

            for (int i = 0; i < list.Count; i++)
            {
                add_to_vec(wrap, ToRustVector2(list[i]));
            }

            graphPtr = allocate_graph(wrap, bounds);
        }

        public static void DeallocGraph()
        {
            if (graphPtr == IntPtr.Zero)
                return;

            UnityEngine.Debug.Log("deallocated");
            deallocate_graph(graphPtr);
            graphPtr = IntPtr.Zero;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static List<Vector2Int> FindPath(UnityEngine.Vector2 start, UnityEngine.Vector2 goal)
        {
            if (graphPtr == IntPtr.Zero)
            {
                UnityEngine.Debug.LogError("Allocate a graph first");
                return null;
            }


            var buf = find_path(ToRustVector2(start), ToRustVector2(goal), graphPtr);

            var list = new List<Vector2Int>();

            for (int i = 0; i < buf.Lenght; ++i)
            {
                var ptr = IntPtr.Add(buf.tilelist, i * 8);
                list.Add(Marshal.PtrToStructure<RustVector2Int>(ptr).ToVector2);
            }

            free_buffer(buf);

            return list;

        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        public static List<Vector2Int> FindPath(Vector2Int start, Vector2Int goal)
        {
            if (graphPtr == IntPtr.Zero)
            {
                UnityEngine.Debug.LogError("Allocate a graph first");
                return null;
            }


            var buf = find_path(ToRustVector2(start), ToRustVector2(goal), graphPtr);

            var list = new List<Vector2Int>();

            for (int i = 0; i < buf.Lenght; ++i)
            {
                var ptr = IntPtr.Add(buf.tilelist, i * 8);
                list.Add(Marshal.PtrToStructure<RustVector2Int>(ptr).ToVector2);
            }

            free_buffer(buf);

            return list;
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal static RustVector2Int ToRustVector2(Vector2Int vector)
        {
            return new RustVector2Int
            {
                x = vector.x,
                y = vector.y
            };
        }

        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        internal static RustVector2Int ToRustVector2(UnityEngine.Vector2 vector)
        {
            return new RustVector2Int
            {
                x = (int)vector.x,
                y = (int)vector.y
            };
        }

        [StructLayout(LayoutKind.Sequential)]
        internal struct RustVector2Int
        {

            public int x;
            public int y;

            public Vector2Int ToVector2
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get => new Vector2Int
                {
                    x = x,
                    y = y
                };
            }
        }

        [StructLayout(LayoutKind.Sequential)]
        internal struct Buffer
        {
            public UIntPtr len;
            public IntPtr tilelist;

            public int Lenght
            {
                [MethodImpl(MethodImplOptions.AggressiveInlining)]
                get => (int)len;
            }

        }

        #region native

        /// 

        [DllImport("rust_pathfind", EntryPoint = "create_vec", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr create_vec();

        [SuppressUnmanagedCodeSecurity]
        [DllImport("rust_pathfind", EntryPoint = "add_to_vec", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void add_to_vec(IntPtr vec_ptr, RustVector2Int pos);

        /// 

        [DllImport("rust_pathfind", EntryPoint = "free_buffer", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void free_buffer(Buffer buf);

        /// 

        [DllImport("rust_pathfind", EntryPoint = "allocate_graph", CallingConvention = CallingConvention.Cdecl)]
        internal static extern IntPtr allocate_graph(IntPtr vec_ptr, Bounds bounds);

        [DllImport("rust_pathfind", EntryPoint = "deallocate_graph", CallingConvention = CallingConvention.Cdecl)]
        internal static extern void deallocate_graph(IntPtr graph_ptr);

        /// 

        [SuppressUnmanagedCodeSecurity]
        [DllImport("rust_pathfind", EntryPoint = "find_path", CallingConvention = CallingConvention.Cdecl)]
        internal static extern Buffer find_path(RustVector2Int start, RustVector2Int goal, IntPtr graph_ptr);
    }
    #endregion
}
