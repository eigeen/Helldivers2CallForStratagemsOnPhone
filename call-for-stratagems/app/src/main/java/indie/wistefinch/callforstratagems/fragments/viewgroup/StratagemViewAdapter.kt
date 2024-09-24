package indie.wistefinch.callforstratagems.fragments.viewgroup

import android.annotation.SuppressLint
import android.content.Context
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ImageView
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView
import indie.wistefinch.callforstratagems.R
import indie.wistefinch.callforstratagems.data.models.StratagemData

/**
 * Adapter for the stratagem recycler view in [ViewGroupFragment]
 */
class StratagemViewAdapter: RecyclerView.Adapter<StratagemViewAdapter.ListViewHolder>() {

    /**
     * All data in the adapter.
     */
    private var dataList = emptyList<StratagemData>()

    /**
     * Context, used to obtain external information.
     */
    private lateinit var context: Context

    class ListViewHolder(itemView: View): RecyclerView.ViewHolder(itemView)

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ListViewHolder {
        context = parent.context
        return ListViewHolder(LayoutInflater.from(parent.context).inflate(R.layout.layout_stratagem_view, parent, false))
    }

    @SuppressLint("DiscouragedApi")
    override fun onBindViewHolder(holder: ListViewHolder, pos: Int) {
        // Set card view text.
        val lang: String = context.resources.configuration.locales.get(0).toLanguageTag()
        holder.itemView.findViewById<TextView>(R.id.stratagem_view_title).text = when (lang) {
            "zh-CN" -> dataList[pos].nameZh
            else -> dataList[pos].name
        }
        // For compatibility with lower SDKs, ignore the discouraged warning.
        val res = context.resources.getIdentifier(
            dataList[pos].icon,
            "drawable",
            context.packageName
        )
        if (res != 0) {
            holder.itemView.findViewById<ImageView>(R.id.stratagem_view_imageView).setImageResource(res)
        }
    }

    override fun getItemCount(): Int {
        return dataList.size
    }

    /**
     * Set the adapter data.
     *
     * Because the data won't change, there is no need to compare the difference, ignore the warning.
     */
    @SuppressLint("NotifyDataSetChanged")
    fun setData(list: List<StratagemData>) {
        this.dataList = list
        notifyDataSetChanged()
    }
}