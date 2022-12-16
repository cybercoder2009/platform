import React, { useState } from 'react'
import { connect } from 'react-redux'
import { Button, Upload, notification, Progress } from 'antd'
import { UploadOutlined } from '@ant-design/icons'
import * as XLSX from 'xlsx'
import { service_items } from '../services.js'

const Uploads = ({ auth, group, lang, langs }) => {
    const [progress_hide, set_progress_hide] = useState(true)
    const [percent, set_percent] = useState(0)
    const check_response = (index, array) => {
        if (index < array.length) {
            service_items.post(auth.token, group, array.slice(index, index + 50))
                .then(_ => {
                    check_response(index + 50, array)
                    if (index + 50 > array.length) {
                        set_percent(100)
                    } else {
                        set_percent((index / (array.length - 1)) * 100)
                    }
                })
        }
    }
    const import_excel = (file) => {
        const file_reader = new FileReader()
        if (file.size < 15000000) {
            file_reader.onload = event => {
                try {
                    /** HARD STOP AS OF SEPT 3 DUE TO SOBEYS UI DESIGN */
                    const result = event.target.result
                    const work_book = XLSX.read(result, { type: "binary" })
                    let data = []
                    for (const Sheet in work_book.Sheets) {
                        XLSX.utils.sheet_to_json(work_book.Sheets[Sheet])
                        if (work_book.Sheets.hasOwnProperty(Sheet)) {
                            data = XLSX.utils.sheet_to_json(work_book.Sheets[Sheet])
                        }
                    }
                    if (group) {
                        set_progress_hide(false)
                        check_response(0, data)
                    } else {
                        notification.open({
                            message: "Issue with Group!",
                            description: "There is no group selected."
                        });
                    }
                } catch (e) {
                    console.log("Failed to read file: ", e)
                }
            }
            file_reader.readAsBinaryString(file)

        } else {
            notification.open({
                message: "File size was over 15MB!",
                description: "Unable to upload file!"
            });
        }
    }
    return (
        <>
            <Upload
                accept=".xlsx, .xls"
                showUploadList={false}
                beforeUpload={file => {
                    import_excel(file)
                    // Prevent upload
                    return false

                }}>
                <Button icon={<UploadOutlined />}>Upload</Button>
            </Upload >
            <Progress hidden={progress_hide} style={{ margin: 10, width: 100 }} percent={parseInt(percent)} format={_ => `${parseInt(percent)} %`} />
        </>
    )
}

export default connect(props => {
    return {
        auth: props.auth,
        group: props.group,
        lang: props.lang,
        langs: props.langs
    }
})(Uploads)